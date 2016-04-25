from flask import Blueprint, jsonify, request
from flask import current_app as app
from decorators import admins_only, api_wrapper, WebException
from models import db, Problems, Files, Config, Users, UserActivity
from schemas import verify_to_schema, check

import logger
import problem
import team
import user
import utils

blueprint = Blueprint("admin", __name__)

@blueprint.route("/setup/init")
@api_wrapper
def admin_setup_init():
	k = Config.query.filter_by(key="setup_verification").first()
	if k is not None and k.value == True: raise WebException("Installation has been complete.")

	verification = Config("setup_verification", utils.generate_string().lower())
	with app.app_context():
		for item in Config.query.filter_by(key="setup_verification").all():
			db.session.delete(item)
		db.session.add(verification)
		db.session.commit()
		return { "success": 1, "verification": verification.value }

@blueprint.route("/setup", methods=["POST"])
@api_wrapper
def admin_setup():
	global user
	params = utils.flat_multi(request.form)

	k = Config.query.filter_by(key="setup_verification").first()
	if k is not None and k.value == True: raise WebException("Installation has been complete.")

	if params.get("verification") != Config.query.filter_by(key="setup_verification").first().value:
		raise WebException("Verification does not match.")

	if params.get("password") != params.get("password_confirm"):
		raise WebException("Passwords do not match.")
	verify_to_schema(user.UserSchema, params)

	name = params.get("name")
	email = params.get("email")
	username = params.get("username")
	password = params.get("password")
	password_confirm = params.get("password_confirm")
	utype = int(params.get("type"))

	setup_vars = [
		Config("ctf_name", params.get("ctf_name")),
		Config("start_time", params.get("start_time")),
		Config("end_time", params.get("end_time")),
		Config("setup_complete", True)
	]

	_user = Users(name, username, email, password, utype=utype, admin=True)
	with app.app_context():
		for var in setup_vars:
			db.session.add(var)

		db.session.add(_user)
		db.session.commit()
		join_activity = UserActivity(_user.uid, 0)
		db.session.add(join_activity)
		db.session.commit()

	logger.log(__name__, "%s registered with %s" % (name.encode("utf-8"), email.encode("utf-8")))
	user.login_user(username, password)

	return { "success": 1, "message": "Success!" }
@blueprint.route("/stats/overview")
@api_wrapper
@admins_only
def admin_stats_overview():
	overview = { }
	overview["num_users"] = user.num_users(), user.num_users(observer=True)
	overview["num_teams"] = team.num_teams(), team.num_teams(observer=True)
	overview["num_problems"] = problem.num_problems()
	return { "success": 1, "overview": overview }

@blueprint.route("/settings")
@api_wrapper
@admins_only
def admin_settings():
	settings_return = {}
	settings = Config.query.all()
	for setting in settings:
		settings_return[setting.key] = setting.value
	return { "success": 1, "settings": settings_return }

@blueprint.route("/settings/update", methods=["POST"])
@api_wrapper
@admins_only
def admin_settings_update():
	params = utils.flat_multi(request.form)
	params.pop("csrf_token")
	with app.app_context():
		for key in params:
			config = Config.query.filter_by(key=key).first()
			new = params[key]
			if config.value != new:
				config.value = params[key]
				db.session.add(config)
		db.session.commit()

	return { "success": 1, "message": "Success!" }
