import time
import re
from io import BytesIO, StringIO

import paramiko
from flask import current_app
from sqlalchemy import and_, func, union_all
from sqlalchemy.ext.hybrid import hybrid_property

import easyctf.models as models
from easyctf.objects import db, cache
from easyctf.constants import USER_REGULAR
from easyctf.utils import generate_identicon, save_file


NEW_USER_PATTERN = re.compile(r"::(user[\d]{5}):([a-zA-Z0-9]+)")


class Team(db.Model):
    __tablename__ = "teams"
    tid = db.Column(db.Integer, primary_key=True, index=True)
    teamname = db.Column(db.Unicode(32), unique=True)
    school = db.Column(db.Unicode(64))
    owner = db.Column(db.Integer)
    classrooms = db.relationship(
        "Classroom", secondary=models.team_classroom, backref="classrooms"
    )
    classroom_invites = db.relationship(
        "Classroom", secondary=models.classroom_invitation, backref="classroom_invites"
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
        secondary=models.team_player_invitation,
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

    # @hybrid_property
    @cache.memoize(timeout=120)
    def observer(self):
        return models.User.query.filter(
            and_(models.User.tid == self.tid, models.User.level != USER_REGULAR)
        ).count()

    # @observer.expression
    # @cache.memoize(timeout=120)
    # def observer(self):
    #     return db.session.query(User).filter(User.tid == self.tid and User.level != USER_REGULAR).count()

    @hybrid_property
    def prop_points(self):
        return sum(
            problem.value
            for problem, solve in db.session.query(models.Problem, models.Solve)
            .filter(models.Solve.tid == self.tid)
            .filter(models.Problem.pid == models.Solve.tid)
            .all()
        )

    @prop_points.expression
    def prop_points_expression(self):
        return (
            db.session.query(models.Problem, models.Solve)
            .filter(models.Solve.tid == self.tid)
            .filter(models.Problem.pid == models.Solve.tid)
            .with_entities(func.sum(models.Problem.value))
            .scalar()
        )

    @cache.memoize(timeout=120)
    def points(self):
        points = 0
        solves = self.solves
        solves.sort(key=lambda s: s.date, reverse=True)
        for solve in solves:
            problem = models.Problem.query.filter_by(pid=solve.pid).first()
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
        solve = (
            models.Solve.query.filter_by(tid=self.tid)
            .order_by(models.Solve.date)
            .first()
        )
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
            return models.Problem.query.order_by(models.Problem.value).all()
        match = models.Problem.value > 0
        if programming is not None:
            match = and_(match, models.Problem.programming == programming)
        problems = (
            models.Problem.query.filter(match).order_by(models.Problem.value).all()
        )
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
            models.Job.query.filter_by(tid=self.tid)
            .order_by(models.Job.completion_time.desc())
            .all()
        )

    def has_solved(self, pid):
        return models.Solve.query.filter_by(tid=self.tid, pid=pid).count() > 0

    @classmethod
    @cache.memoize(timeout=60)
    def scoreboard(cls):
        # credit: https://github.com/CTFd/CTFd/blob/master/CTFd/scoreboard.py
        uniq = (
            db.session.query(
                models.Solve.tid.label("tid"), models.Solve.pid.label("pid")
            )
            .distinct()
            .subquery()
        )
        # flash("uniq: " + str(uniq).replace("\n", ""), "info")
        scores = (
            db.session.query(
                # uniq.columns.tid.label("tid"),
                models.Solve.tid.label("tid"),
                db.func.max(models.Solve.pid).label("pid"),
                db.func.sum(models.Problem.value).label("score"),
                db.func.max(models.Solve.date).label("date"),
            )
            .join(models.Problem)
            .group_by(models.Solve.tid)
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
        start_time = int(models.Config.get("start_time", default=0))
        for solve in solves:
            score += solve.problem.value
            frame = [convert_to_time(solve.date - start_time), score]
            progression.append(frame)

        progression.append([convert_to_time(time.time() - start_time), score])

        return progression

    def credentials(self):
        host = current_app.config.get("SHELL_HOST")
        if not host:
            return None
        print("host:", host)
        private_key_contents, _ = models.Config.get_ssh_keys()
        private_key = paramiko.Ed25519Key.from_private_key(
            file_obj=StringIO(private_key_contents)
        )
        if not private_key:
            return None
        print("private key:", private_key)
        if not self.shell_user or not self.shell_pass:
            client = paramiko.SSHClient()
            client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
            client.connect(
                host, username="ctfadmin", pkey=private_key, look_for_keys=False
            )
            _, stdout, _ = client.exec_command("\n")
            data = stdout.read().decode("utf-8").split("\n")
            for line in data:
                match = NEW_USER_PATTERN.match(line)
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
