import base64
import os
import time
from io import BytesIO
from datetime import datetime

import onetimepass
from passlib.hash import bcrypt
from sqlalchemy.ext.hybrid import hybrid_property

import easyctf.models as models
from easyctf.objects import cache, db, login_manager
from easyctf.utils import generate_identicon, save_file


class User(db.Model):
    """
    User model.
    """

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
        secondary=models.player_team_invitation,
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
    def password_setter(self, password):
        self._password = bcrypt.encrypt(password, rounds=10)

    @hybrid_property
    def register_time(self):
        return int(time.mktime(self._register_time.timetuple()))

    @hybrid_property
    def username_lower(self):
        return self.username.lower()

    def get_totp_uri(self):
        if self.otp_secret is None:
            # TODO: Replace with secrets library
            secret = base64.b32encode(os.urandom(10)).decode("utf-8").lower()
            self.otp_secret = secret
            db.session.add(self)
            db.session.commit()
        service_name = models.Config.get("ctf_name")
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
