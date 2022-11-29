import imp
import os
import re
import sys
import time
from datetime import datetime
import traceback
from string import Template
from typing import Tuple

import yaml
from Crypto.PublicKey import ECC
from flask import current_app as app, url_for
from flask_login import current_user
from markdown2 import markdown
from sqlalchemy import func
from sqlalchemy.ext.hybrid import hybrid_property

from easyctf.config import Config as AppConfig
from easyctf.objects import cache, db, login_manager
from easyctf.utils import (
    generate_short_string,
    generate_string,
    save_file,
)

config = AppConfig()
SEED = "OPENCTF_PROBLEM_SEED_PREFIX_%s" % config.SECRET_KEY
login_manager.login_view = "users.login"
login_manager.login_message_category = "danger"


def filename_filter(name):
    return re.sub("[^a-zA-Z0-9]+", "_", name)


team_classroom = db.Table(
    "team_classroom",
    db.Column("team_id", db.Integer, db.ForeignKey("teams.tid"), nullable=False),
    db.Column(
        "classroom_id", db.Integer, db.ForeignKey("classrooms.id"), nullable=False
    ),
    db.PrimaryKeyConstraint("team_id", "classroom_id"),
)
classroom_invitation = db.Table(
    "classroom_invitation",
    db.Column("team_id", db.Integer, db.ForeignKey("teams.tid"), nullable=False),
    db.Column(
        "classroom_id", db.Integer, db.ForeignKey("classrooms.id"), nullable=False
    ),
    db.PrimaryKeyConstraint("team_id", "classroom_id"),
)

team_player_invitation = db.Table(
    "team_player_invitation",
    db.Column("team_id", db.Integer, db.ForeignKey("teams.tid", primary_key=True)),
    db.Column("user_id", db.Integer, db.ForeignKey("users.uid", primary_key=True)),
)

player_team_invitation = db.Table(
    "player_team_invitation",
    db.Column("user_id", db.Integer, db.ForeignKey("users.uid", primary_key=True)),
    db.Column("team_id", db.Integer, db.ForeignKey("teams.tid", primary_key=True)),
)


class Config(db.Model):
    __tablename__ = "config"
    cid = db.Column(db.Integer, primary_key=True)
    key = db.Column(db.Unicode(32), index=True)
    value = db.Column(db.Text)

    def __init__(self, key, value):
        self.key = key
        self.value = value

    @classmethod
    def get_competition_window(cls):
        return (0, 0)

    @classmethod
    def get_team_size(cls):
        # TODO: actually implement this
        return 5

    @classmethod
    def get(cls, key, default=None):
        config = cls.query.filter_by(key=key).first()
        if config is None:
            return default
        return str(config.value)

    @classmethod
    def set(cls, key, value):
        config = cls.query.filter_by(key=key).first()
        if config is None:
            config = Config(key, value)
        db.session.add(config)
        db.session.commit()

    @classmethod
    def get_many(cls, keys):
        items = cls.query.filter(cls.key.in_(keys)).all()
        return dict([(item.key, item.value) for item in items])

    @classmethod
    def set_many(cls, configs):
        for key, value in list(configs.items()):
            config = cls.query.filter_by(key=key).first()
            if config is None:
                config = Config(key, value)
            config.value = value
            db.session.add(config)
        db.session.commit()

    @classmethod
    def get_ssh_keys(cls) -> Tuple[str, str]:
        private_key = cls.get("private_key")
        public_key = cls.get("public_key")
        if not (private_key and public_key):
            # key = RSA.generate(2048)
            key = ECC.generate(curve="ed25519")
            private_key = key.export_key(format="PEM")
            public_key = key.public_key().export_key(format="OpenSSH")
            cls.set_many(
                {
                    "private_key": private_key,
                    "public_key": public_key,
                }
            )
        return private_key, public_key

    def __repr__(self):
        return "Config({}={})".format(self.key, self.value)


class Problem(db.Model):
    __tablename__ = "problems"
    pid = db.Column(db.Integer, index=True, primary_key=True)
    author = db.Column(db.Unicode(32))
    name = db.Column(db.String(32), unique=True)
    title = db.Column(db.Unicode(64))
    description = db.Column(db.Text)
    hint = db.Column(db.Text)
    category = db.Column(db.Unicode(64))
    value = db.Column(db.Integer)

    grader = db.Column(db.UnicodeText)
    autogen = db.Column(db.Boolean)
    programming = db.Column(db.Boolean)
    threshold = db.Column(db.Integer)
    weightmap = db.Column(db.PickleType)
    solves = db.relationship("Solve", backref="problem", lazy=True)
    jobs = db.relationship("Job", backref="problem", lazy=True)

    test_cases = db.Column(db.Integer)
    time_limit = db.Column(db.Integer)  # in seconds
    memory_limit = db.Column(db.Integer)  # in kb
    generator = db.Column(db.Text)
    # will use the same grader as regular problems
    source_verifier = db.Column(db.Text)  # may be implemented (possibly)
    path = db.Column(db.String(128))  # path to problem source code

    files = db.relationship("File", backref="problem", lazy=True)
    autogen_files = db.relationship("AutogenFile", backref="problem", lazy=True)

    @staticmethod
    def validate_problem(path, name):
        files = os.listdir(path)
        valid = True
        for required_file in ["grader.py", "problem.yml", "description.md"]:
            if required_file not in files:
                print("\t* Missing {}".format(required_file))
                valid = False

        if not valid:
            return valid

        metadata = yaml.load(open(os.path.join(path, "problem.yml")))
        if metadata.get("programming", False):
            if "generator.py" not in files:
                print("\t* Missing generator.py")
                valid = False

            for required_key in ["test_cases", "time_limit", "memory_limit"]:
                if required_key not in metadata:
                    print(
                        "\t* Expected required key {} in 'problem.yml'".format(
                            required_key
                        )
                    )
                    valid = False

        return valid

    @staticmethod
    def import_problem(path, name):
        print(" - {}".format(name))
        if not Problem.validate_problem(path, name):
            return

        problem = Problem.query.filter_by(name=name).first()
        if not problem:
            problem = Problem(name=name)

        metadata = yaml.load(open(os.path.join(path, "problem.yml")))
        problem.author = metadata.get("author", "")
        problem.title = metadata.get("title", "")
        problem.category = metadata.get("category", "")
        problem.value = int(metadata.get("value", "0"))
        problem.hint = metadata.get("hint", "")
        problem.autogen = metadata.get("autogen", False)
        problem.programming = metadata.get("programming", False)

        problem.description = open(os.path.join(path, "description.md")).read()
        problem.grader = open(os.path.join(path, "grader.py")).read()
        problem.path = os.path.realpath(path)

        if metadata.get("threshold") and type(metadata.get("threshold")) is int:
            problem.threshold = metadata.get("threshold")
            problem.weightmap = metadata.get("weightmap", {})

        if problem.programming:
            problem.test_cases = int(metadata.get("test_cases"))
            problem.time_limit = int(metadata.get("time_limit"))
            problem.memory_limit = int(metadata.get("memory_limit"))
            problem.generator = open(os.path.join(path, "generator.py")).read()

        db.session.add(problem)
        db.session.flush()
        db.session.commit()

        files = metadata.get("files", [])
        for filename in files:
            file_path = os.path.join(path, filename)
            if not os.path.isfile(file_path):
                print("\t* File '{}' doesn't exist".format(filename, name))
                continue

            source = open(file_path, "rb")
            file = File(pid=problem.pid, filename=filename, data=source)

            existing = File.query.filter_by(pid=problem.pid, filename=filename).first()
            # Update existing file url
            if existing:
                existing.url = file.url
                db.session.add(existing)
            else:
                db.session.add(file)

        db.session.commit()

    @staticmethod
    def import_repository(path):
        if not (
            os.path.realpath(path) and os.path.exists(path) and os.path.isdir(path)
        ):
            print("this isn't a path")
            sys.exit(1)
        path = os.path.realpath(path)
        names = os.listdir(path)
        for name in names:
            if name.startswith("."):
                continue
            problem_dir = os.path.join(path, name)
            if not os.path.isdir(problem_dir):
                continue
            problem_name = os.path.basename(problem_dir)
            Problem.import_problem(problem_dir, problem_name)

    @classmethod
    def categories(cls):
        def f(c):
            return c[0]

        categories = map(f, db.session.query(Problem.category).distinct().all())
        return list(categories)

    @staticmethod
    def get_by_id(id):
        query_results = Problem.query.filter_by(pid=id)
        return query_results.first() if query_results.count() else None

    @property
    def solved(self):
        return Solve.query.filter_by(pid=self.pid, tid=current_user.tid).count()

    def get_grader(self):
        grader = imp.new_module("grader")
        curr = os.getcwd()
        if self.path:
            os.chdir(self.path)
        exec(self.grader, grader.__dict__)
        os.chdir(curr)
        return grader

    def get_autogen(self, tid):
        autogen = __import__("random")
        autogen.seed("%s_%s_%s" % (SEED, self.pid, tid))
        return autogen

    def render_description(self, tid):
        description = markdown(self.description, extras=["fenced-code-blocks"])
        try:
            variables = {}
            template = Template(description)
            if self.autogen:
                autogen = self.get_autogen(tid)
                grader = self.get_grader()
                generated_problem = grader.generate(autogen)
                if "variables" in generated_problem:
                    variables.update(generated_problem["variables"])
                if "files" in generated_problem:
                    for file in generated_problem["files"]:
                        url = url_for("chals.autogen", pid=self.pid, filename=file)
                        variables[File.clean_name(file)] = url
            static_files = File.query.filter_by(pid=self.pid).all()
            if static_files is not None:
                for file in static_files:
                    url = "{}/{}".format(app.config["FILESTORE_STATIC"], file.url)
                    variables[File.clean_name(file.filename)] = url
            description = template.safe_substitute(variables)
        except Exception as e:
            description += "<!-- parsing error: {} -->".format(traceback.format_exc())
            traceback.print_exc(file=sys.stderr)
        description = description.replace("${", "{")
        return description

    # TODO: clean up the shitty string-enum return
    # the shitty return is used directly in game.py
    def try_submit(self, flag):
        solved = Solve.query.filter_by(tid=current_user.tid, pid=self.pid).first()
        if solved:
            return "error", "You've already solved this problem"
        already_tried = WrongFlag.query.filter_by(
            tid=current_user.tid, pid=self.pid, flag=flag
        ).count()
        if already_tried:
            return "error", "You've already tried this flag"
        random = None
        if self.autogen:
            random = self.get_autogen(current_user.tid)

        grader = self.get_grader()
        correct, message = grader.grade(random, flag)
        if correct:
            submission = Solve(
                pid=self.pid, tid=current_user.tid, uid=current_user.uid, flag=flag
            )
            db.session.add(submission)
            db.session.commit()
        else:
            if len(flag) < 256:
                submission = WrongFlag(
                    pid=self.pid, tid=current_user.tid, uid=current_user.uid, flag=flag
                )
                db.session.add(submission)
                db.session.commit()
            else:
                # fuck you
                pass

        cache.delete_memoized(current_user.team.place)
        cache.delete_memoized(current_user.team.points)
        cache.delete_memoized(current_user.team.get_last_solved)
        cache.delete_memoized(current_user.team.get_score_progression)

        return "success" if correct else "failure", message

    def api_summary(self):
        summary = {
            field: getattr(self, field)
            for field in [
                "pid",
                "author",
                "name",
                "title",
                "hint",
                "category",
                "value",
                "solved",
                "programming",
            ]
        }
        summary["description"] = self.render_description(current_user.tid)
        return summary


class File(db.Model):
    __tablename__ = "files"
    id = db.Column(db.Integer, index=True, primary_key=True)
    pid = db.Column(db.Integer, db.ForeignKey("problems.pid"), index=True)
    filename = db.Column(db.Unicode(64))
    url = db.Column(db.String(128))

    @staticmethod
    def clean_name(name):
        return filename_filter(name)

    def __init__(self, pid, filename, data):
        self.pid = pid
        self.filename = filename
        data.seek(0)
        if not app.config.get("TESTING"):
            response = save_file(data, suffix="_" + filename)
            if response.status_code == 200:
                self.url = response.text


class AutogenFile(db.Model):
    __tablename__ = "autogen_files"
    id = db.Column(db.Integer, index=True, primary_key=True)
    pid = db.Column(db.Integer, db.ForeignKey("problems.pid"), index=True)
    tid = db.Column(db.Integer, index=True)
    filename = db.Column(db.Unicode(64), index=True)
    url = db.Column(db.String(128))

    @staticmethod
    def clean_name(name):
        return filename_filter(name)

    def __init__(self, pid, tid, filename, data):
        self.pid = pid
        self.tid = tid
        self.filename = filename
        data.seek(0)
        if not app.config.get("TESTING"):
            response = save_file(data, suffix="_" + filename)
            if response.status_code == 200:
                self.url = response.text


class PasswordResetToken(db.Model):
    __tablename__ = "password_reset_tokens"
    id = db.Column(db.Integer, primary_key=True)
    uid = db.Column(db.Integer, db.ForeignKey("users.uid"), index=True)
    active = db.Column(db.Boolean)
    token = db.Column(db.String(32), default=generate_short_string)
    email = db.Column(db.Unicode(128))
    expire = db.Column(db.DateTime)

    @property
    def expired(self):
        return datetime.utcnow() >= self.expire

    @property
    def user(self):
        return User.get_by_id(self.uid)


class Solve(db.Model):
    __tablename__ = "solves"
    __table_args__ = (db.UniqueConstraint("pid", "tid"),)
    id = db.Column(db.Integer, index=True, primary_key=True)
    pid = db.Column(db.Integer, db.ForeignKey("problems.pid"), index=True)
    tid = db.Column(db.Integer, db.ForeignKey("teams.tid"), index=True)
    uid = db.Column(db.Integer, db.ForeignKey("users.uid"), index=True)
    _date = db.Column("date", db.DateTime, default=datetime.utcnow)
    flag = db.Column(db.Unicode(256))

    @hybrid_property
    def date(self):
        return int(time.mktime(self._date.timetuple()))

    @date.expression
    def date_expression(self):
        return self._date


class WrongFlag(db.Model):
    __tablename__ = "wrong_flags"
    id = db.Column(db.Integer, index=True, primary_key=True)
    pid = db.Column(db.Integer, db.ForeignKey("problems.pid"), index=True)
    tid = db.Column(db.Integer, db.ForeignKey("teams.tid"), index=True)
    uid = db.Column(db.Integer, db.ForeignKey("users.uid"), index=True)
    _date = db.Column("date", db.DateTime, default=datetime.utcnow)
    flag = db.Column(db.Unicode(256), index=True)

    @hybrid_property
    def date(self):
        return int(time.mktime(self._date.timetuple()))

    @date.expression
    def date_expression(self):
        return self._date


class Classroom(db.Model):
    __tablename__ = "classrooms"
    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.Unicode(64), nullable=False)
    owner = db.Column(db.Integer)
    teams = db.relationship(
        "Team", passive_deletes=True, secondary=team_classroom, backref="teams"
    )
    invites = db.relationship(
        "Team", passive_deletes=True, secondary=classroom_invitation, backref="invites"
    )

    def __contains__(self, obj):
        if isinstance(obj, Team):
            return obj in self.teams
        return False

    @property
    def teacher(self):
        return User.query.filter_by(uid=self.owner).first()

    @property
    def size(self):
        return len(self.teams)

    @property
    def scoreboard(self):
        return sorted(
            self.teams,
            key=lambda team: (team.points(), -team.get_last_solved()),
            reverse=True,
        )


class Egg(db.Model):
    __tablename__ = "eggs"
    eid = db.Column(db.Integer, primary_key=True)
    flag = db.Column(db.Unicode(64), nullable=False, unique=True, index=True)
    solves = db.relationship("EggSolve", backref="egg", lazy=True)


class EggSolve(db.Model):
    __tablename__ = "egg_solves"
    sid = db.Column(db.Integer, primary_key=True)
    eid = db.Column(db.Integer, db.ForeignKey("eggs.eid"), index=True)
    tid = db.Column(db.Integer, db.ForeignKey("teams.tid"), index=True)
    uid = db.Column(db.Integer, db.ForeignKey("users.uid"), index=True)
    date = db.Column(db.DateTime, default=datetime.utcnow)


class WrongEgg(db.Model):
    __tablename__ = "wrong_egg"
    id = db.Column(db.Integer, primary_key=True)
    eid = db.Column(db.Integer, db.ForeignKey("eggs.eid"), index=True)
    tid = db.Column(db.Integer, db.ForeignKey("teams.tid"), index=True)
    uid = db.Column(db.Integer, db.ForeignKey("users.uid"), index=True)
    date = db.Column(db.DateTime, default=datetime.utcnow)
    submission = db.Column(db.Unicode(64))


# judge stuff


class JudgeKey(db.Model):
    __tablename__ = "judge_api_keys"
    id = db.Column(db.Integer, primary_key=True)
    key = db.Column(db.String(64), index=True, default=generate_string)
    ip = db.Column(db.Integer)  # maybe?


class Job(db.Model):
    __tablename__ = "jobs"
    id = db.Column(db.Integer, primary_key=True)
    pid = db.Column(db.Integer, db.ForeignKey("problems.pid"), index=True)
    tid = db.Column(db.Integer, db.ForeignKey("teams.tid"), index=True)
    uid = db.Column(db.Integer, db.ForeignKey("users.uid"), index=True)

    submitted = db.Column(db.DateTime, default=datetime.utcnow)
    claimed = db.Column(db.DateTime)
    completed = db.Column(db.DateTime)

    execution_time = db.Column(db.Float)
    execution_memory = db.Column(db.Float)

    language = db.Column(db.String(16), nullable=False)
    contents = db.Column(db.Text, nullable=False)
    feedback = db.Column(db.Text)

    # 0 = waiting
    # 1 = progress
    # 2 = done
    # 3 = errored
    status = db.Column(db.Integer, index=True, nullable=False, default=0)

    verdict = db.Column(db.String(8))
    last_ran_case = db.Column(db.Integer)


class GameState(db.Model):
    __tablename__ = "game_states"
    id = db.Column(db.Integer, primary_key=True)
    uid = db.Column(db.Integer, db.ForeignKey("users.uid"), unique=True)
    last_updated = db.Column(
        db.DateTime,
        server_default=func.now(),
        onupdate=func.current_timestamp(),
        unique=True,
    )

    state = db.Column(db.UnicodeText, nullable=False, default="{}")
