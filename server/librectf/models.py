import base64
import imp
import os
import re
import sys
import time
from datetime import datetime
import traceback
from io import BytesIO, StringIO
from string import Template

import onetimepass
import paramiko
import yaml
from Crypto.PublicKey import RSA
from flask import current_app as app
from flask import url_for
from flask_login import current_user
from markdown2 import markdown
from passlib.hash import bcrypt
from sqlalchemy import and_, func, select
from sqlalchemy.ext.hybrid import hybrid_property
from sqlalchemy.orm import backref
from sqlalchemy.sql.expression import union_all

from librectf.constants import USER_REGULAR
from librectf.objects import cache, db, login_manager
from librectf.utils import (
    generate_identicon,
    generate_short_string,
    generate_string,
    save_file,
)

# TODO: randomize
SEED = "OPENCTF_PROBLEM_SEED_PREFIX_%s"
login_manager.login_view = "users.login"
login_manager.login_message_category = "danger"

new_user_pattern = re.compile(r"::(user[\d]{5}):([a-zA-Z0-9]+)")


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
    def get_ssh_keys(cls):
        private_key = cls.get("private_key")
        public_key = cls.get("public_key")
        if not (private_key and public_key):
            key = RSA.generate(2048)
            private_key = key.exportKey("PEM")
            public_key = key.publickey().exportKey("OpenSSH")
            cls.set_many(
                {
                    "private_key": str(private_key, "utf-8"),
                    "public_key": str(public_key, "utf-8"),
                }
            )
        return private_key, public_key

    def __repr__(self):
        return "Config({}={})".format(self.key, self.value)


class User(db.Model):
    __tablename__ = "users"
    uid = db.Column(db.Integer, index=True, primary_key=True)
    tid = db.Column(db.Integer, db.ForeignKey("teams.tid"))
    name = db.Column(db.Unicode(32))
    easyctf = db.Column(db.Boolean, index=True, default=False)
    username = db.Column(db.String(16), unique=True, index=True)
    email = db.Column(db.String(128), unique=True)
    _password = db.Column("password", db.String(128))
    admin = db.Column(db.Boolean, default=False)
    level = db.Column(db.Integer)
    _register_time = db.Column("register_time", db.DateTime, default=datetime.utcnow)
    reset_token = db.Column(db.String(32))
    otp_secret = db.Column(db.String(16))
    otp_confirmed = db.Column(db.Boolean, default=False)
    email_token = db.Column(db.String(32))
    email_verified = db.Column(db.Boolean, default=False)

    team = db.relationship("Team", back_populates="members")
    solves = db.relationship("Solve", backref="user", lazy=True)
    jobs = db.relationship("Job", backref="user", lazy=True)
    _avatar = db.Column("avatar", db.String(128))

    outgoing_invitations = db.relationship(
        "Team",
        secondary=player_team_invitation,
        lazy="subquery",
        backref=db.backref("incoming_invitations", lazy=True),
    )

    @property
    def avatar(self):
        if not self._avatar:
            avatar_file = BytesIO()
            avatar = generate_identicon("user%s" % self.uid)
            avatar.save(avatar_file, format="PNG")
            avatar_file.seek(0)
            response = save_file(avatar_file, prefix="team_avatar_", suffix=".png")
            if response.status_code == 200:
                self._avatar = response.text
                db.session.add(self)
                db.session.commit()
        return self._avatar or ""  # just so the frontend doesnt 500

    def __eq__(self, other):
        if isinstance(other, User):
            return self.uid == other.uid
        return NotImplemented

    def __str__(self):
        return "<User %s>" % self.uid

    def check_password(self, password):
        return bcrypt.verify(password, self.password)

    def get_id(self):
        return str(self.uid)

    @property
    def is_anonymous(self):
        return False

    @staticmethod
    @login_manager.user_loader
    def get_by_id(id):
        query_results = User.query.filter_by(uid=id)
        return query_results.first()

    @property
    def is_active(self):
        # TODO This will be based off account standing.
        return True

    @property
    def is_authenticated(self):
        return True

    @hybrid_property
    def password(self):
        return self._password

    @password.setter
    def password(self, password):
        self._password = bcrypt.encrypt(password, rounds=10)

    @hybrid_property
    def register_time(self):
        return int(time.mktime(self._register_time.timetuple()))

    @hybrid_property
    def username_lower(self):
        return self.username.lower()

    def get_totp_uri(self):
        if self.otp_secret is None:
            secret = base64.b32encode(os.urandom(10)).decode("utf-8").lower()
            self.otp_secret = secret
            db.session.add(self)
            db.session.commit()
        service_name = Config.get("ctf_name")
        return "otpauth://totp/%s:%s?secret=%s&issuer=%s" % (
            service_name,
            self.username,
            self.otp_secret,
            service_name,
        )

    def verify_totp(self, token):
        return onetimepass.valid_totp(token, self.otp_secret)

    @cache.memoize(timeout=120)
    def points(self):
        points = 0
        for solve in self.solves:
            points += solve.problem.value
        return points


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
                print(f"\t* File '{filename}' doesn't exist")
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


class Team(db.Model):
    __tablename__ = "teams"
    tid = db.Column(db.Integer, primary_key=True, index=True)
    teamname = db.Column(db.Unicode(32), unique=True)
    school = db.Column(db.Unicode(64))
    owner = db.Column(db.Integer)
    classrooms = db.relationship(
        "Classroom", secondary=team_classroom, backref="classrooms"
    )
    classroom_invites = db.relationship(
        "Classroom", secondary=classroom_invitation, backref="classroom_invites"
    )
    members = db.relationship("User", back_populates="team")
    admin = db.Column(db.Boolean, default=False)
    shell_user = db.Column(db.String(16), unique=True)
    shell_pass = db.Column(db.String(32))
    banned = db.Column(db.Boolean, default=False)
    solves = db.relationship("Solve", backref="team", lazy=True)
    jobs = db.relationship("Job", backref="team", lazy=True)
    _avatar = db.Column("avatar", db.String(128))

    outgoing_invitations = db.relationship(
        "User",
        secondary=team_player_invitation,
        lazy="subquery",
        backref=db.backref("incoming_invitations", lazy=True),
    )

    def __repr__(self):
        return "%s_%s" % (self.__class__.__name__, self.tid)

    def __str__(self):
        return "<Team %s>" % self.tid

    @property
    def avatar(self):
        if not self._avatar:
            avatar_file = BytesIO()
            avatar = generate_identicon("team%s" % self.tid)
            avatar.save(avatar_file, format="PNG")
            avatar_file.seek(0)
            response = save_file(avatar_file, prefix="user_avatar_", suffix=".png")
            if response.status_code == 200:
                self._avatar = response.text
                db.session.add(self)
                db.session.commit()
        return self._avatar

    @staticmethod
    def get_by_id(id):
        query_results = Team.query.filter_by(tid=id)
        return query_results.first()

    @property
    def size(self):
        return len(self.members)

    @hybrid_property
    def observer(self):
        return User.query.filter(
            and_(User.tid == self.tid, User.level != USER_REGULAR)
        ).count()

    @observer.expression
    def observer_expr(self):
        return (
            db.session.query(User)
            .filter(User.tid == self.tid and User.level != USER_REGULAR)
            .count()
        )

    @hybrid_property
    def prop_points(self):
        return sum(
            problem.value
            for problem, solve in db.session.query(Problem, Solve)
            .filter(Solve.tid == self.tid)
            .filter(Problem.pid == Solve.tid)
            .all()
        )

    @prop_points.expression
    def prop_points_expr(self):
        return (
            db.session.query(Problem, Solve)
            .filter(Solve.tid == self.tid)
            .filter(Problem.pid == Solve.tid)
            .with_entities(func.sum(Problem.value))
            .scalar()
        )

    @cache.memoize(timeout=120)
    def points(self):
        points = 0
        solves = self.solves
        solves.sort(key=lambda s: s.date, reverse=True)
        for solve in solves:
            problem = Problem.query.filter_by(pid=solve.pid).first()
            points += problem.value
        return points

    @cache.memoize(timeout=120)
    def place(self):
        scoreboard = Team.scoreboard()
        if not self.observer:
            scoreboard = [team for team in scoreboard if not team.observer]
        i = 0
        for i in range(len(scoreboard)):
            if scoreboard[i].tid == self.tid:
                break
        i += 1
        return i

    @hybrid_property
    def prop_last_solved(self):
        solve = Solve.query.filter_by(tid=self.tid).order_by(Solve.date).first()
        if not solve:
            return 0
        return solve.date

    @cache.memoize(timeout=120)
    def get_last_solved(self):
        solves = self.solves
        solves.sort(key=lambda s: s.date, reverse=True)
        if solves:
            solve = solves[0]
            return solve.date if solve else 0
        return 0

    def has_unlocked(self, problem):
        solves = self.solves
        if not problem.weightmap:
            return True
        current = sum(
            [problem.weightmap.get(solve.problem.name, 0) for solve in solves]
        )
        return current >= problem.threshold

    def get_unlocked_problems(self, admin=False, programming=None):
        if admin:
            return Problem.query.order_by(Problem.value).all()
        match = Problem.value > 0
        if programming is not None:
            match = and_(match, Problem.programming == programming)
        problems = Problem.query.filter(match).order_by(Problem.value).all()
        solves = self.solves

        def unlocked(problem):
            if not problem.weightmap:
                return True
            current = sum(
                [problem.weightmap.get(solve.problem.name, 0) for solve in solves]
            )
            return current >= problem.threshold

        return list(filter(unlocked, problems))

    def get_jobs(self):
        return (
            Job.query.filter_by(tid=self.tid).order_by(Job.completion_time.desc()).all()
        )

    def has_solved(self, pid):
        return Solve.query.filter_by(tid=self.tid, pid=pid).count() > 0

    @classmethod
    @cache.memoize(timeout=60)
    def scoreboard(cls):
        # credit: https://github.com/CTFd/CTFd/blob/master/CTFd/scoreboard.py
        uniq = (
            db.session.query(Solve.tid.label("tid"), Solve.pid.label("pid"))
            .distinct()
            .subquery()
        )
        # flash("uniq: " + str(uniq).replace("\n", ""), "info")
        scores = (
            db.session.query(
                # uniq.columns.tid.label("tid"),
                Solve.tid.label("tid"),
                db.func.max(Solve.pid).label("pid"),
                db.func.sum(Problem.value).label("score"),
                db.func.max(Solve.date).label("date"),
            )
            .join(Problem)
            .group_by(Solve.tid)
        )
        # flash("scores: " + str(scores).replace("\n", ""), "info")
        results = union_all(scores).alias("results")
        sumscores = (
            db.session.query(
                results.columns.tid,
                db.func.sum(results.columns.score).label("score"),
                db.func.max(results.columns.pid),
                db.func.max(results.columns.date).label("date"),
            )
            .group_by(results.columns.tid)
            .subquery()
        )
        query = (
            db.session.query(
                Team,
                Team.tid.label("tid"),
                sumscores.columns.score,
                sumscores.columns.date,
            )
            .filter(Team.banned == False)
            .join(sumscores, Team.tid == sumscores.columns.tid)
            .order_by(sumscores.columns.score.desc(), sumscores.columns.date)
        )
        # flash("full query: " + str(query).replace("\n", ""), "info")
        return query.all()

    @cache.memoize(timeout=120)
    def get_score_progression(self):
        def convert_to_time(time):
            m, s = divmod(time, 60)
            h, m = divmod(m, 60)
            d, h = divmod(h, 24)
            if d > 0:
                return "%d:%02d:%02d:%02d" % (d, h, m, s)
            return "%d:%02d:%02d" % (h, m, s)

        solves = self.solves
        solves.sort(key=lambda s: s.date)
        progression = [["Time", "Score"], [convert_to_time(0), 0]]
        score = 0
        start_time = int(Config.get("start_time", default=0))
        for solve in solves:
            score += solve.problem.value
            frame = [convert_to_time(solve.date - start_time), score]
            progression.append(frame)

        progression.append([convert_to_time(time.time() - start_time), score])

        return progression

    def credentials(self):
        host = app.config.get("SHELL_HOST")
        if not host:
            return None
        print("host:", host)
        private_key_contents, _ = Config.get_ssh_keys()
        private_key = paramiko.rsakey.RSAKey(file_obj=StringIO(private_key_contents))
        if not private_key:
            return None
        print("private key:", private_key)
        if not self.shell_user or not self.shell_pass:
            client = paramiko.SSHClient()
            client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
            client.connect(
                host, username="ctfadmin", pkey=private_key, look_for_keys=False
            )
            stdin, stdout, stderr = client.exec_command("\n")
            data = stdout.read().decode("utf-8").split("\n")
            for line in data:
                match = new_user_pattern.match(line)
                if match:
                    username, password = match.group(1), match.group(2)
                    break
            else:
                return None
            self.shell_user = username
            self.shell_pass = password
            db.session.commit()
            return (username, password)
        return (self.shell_user, self.shell_pass)


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
