from datetime import datetime
import time
import logging
import socket

from flask import Flask, request
from flask_login import current_user
from flask_migrate import Migrate


def create_app(config=None):
    app = Flask(__name__, static_folder="assets", static_url_path="/assets")
    hostname = socket.gethostname()

    print("CONFIG IS", config)
    if config is None:
        from easyctf.config import Config

        config = Config()
    app.config.from_object(config)

    from easyctf.objects import cache, db, login_manager, sentry
    import easyctf.models

    cache.init_app(app)
    db.init_app(app)
    login_manager.init_app(app)
    if app.config.get("ENVIRONMENT") != "development":
        sentry.init_app(app, logging=True, level=logging.WARNING)

    from easyctf.utils import filestore, to_place_str, to_timestamp

    app.jinja_env.globals.update(filestore=filestore)
    app.jinja_env.filters["to_timestamp"] = to_timestamp
    app.jinja_env.filters["to_place_str"] = to_place_str

    from easyctf.models import Config

    Migrate(app, db)

    def get_competition_running():
        configs = Config.get_many(["start_time", "end_time"])
        if "start_time" not in configs or "end_time" not in configs:
            return None, None, False
        start_time_str = configs["start_time"]
        end_time_str = configs["end_time"]

        start_time = datetime.fromtimestamp(float(start_time_str))
        end_time = datetime.fromtimestamp(float(end_time_str))
        now = datetime.utcnow()

        competition_running = start_time < now and now < end_time
        return start_time, end_time, competition_running

    @app.after_request
    def easter_egg_link(response):
        if not request.cookies.get("easter_egg_enabled"):
            response.set_cookie("easter_egg_enabled", "0")
        return response

    # TODO: actually finish this
    @app.context_processor
    def inject_config():
        (
            competition_start,
            competition_end,
            competition_running,
        ) = get_competition_running()
        easter_egg_enabled = False
        if competition_running and current_user.is_authenticated:
            try:
                easter_egg_enabled = int(request.cookies.get("easter_egg_enabled")) == 1
            except:
                pass
        config = dict(
            admin_email="",
            hostname=hostname,
            competition_running=competition_running,
            competition_start=competition_start,
            competition_end=competition_end,
            ctf_name=Config.get("ctf_name", "OpenCTF"),
            easter_egg_enabled=easter_egg_enabled,
            environment=app.config.get("ENVIRONMENT", "production"),
        )
        return config

    from easyctf.views import admin, base, classroom, chals, game, judge, teams, users

    app.register_blueprint(admin.blueprint, url_prefix="/admin")
    app.register_blueprint(base.blueprint)
    app.register_blueprint(classroom.blueprint, url_prefix="/classroom")
    app.register_blueprint(chals.blueprint, url_prefix="/chals")
    app.register_blueprint(game.blueprint, url_prefix="/game")
    app.register_blueprint(judge.blueprint, url_prefix="/judge")
    app.register_blueprint(teams.blueprint, url_prefix="/teams")
    app.register_blueprint(users.blueprint, url_prefix="/users")

    return app
