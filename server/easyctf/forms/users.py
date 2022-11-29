from flask_login import current_user
from flask_wtf import FlaskForm
from sqlalchemy import func
from wtforms import ValidationError
from wtforms.fields import (
    BooleanField,
    FileField,
    IntegerField,
    PasswordField,
    RadioField,
    StringField,
    SubmitField,
)
from wtforms.validators import Email, EqualTo, InputRequired, Length, Optional
from wtforms.widgets import NumberInput

from easyctf.forms.validators import UsernameLengthValidator
from easyctf.models import User
from easyctf.utils import VALID_USERNAME


class ChangeLoginForm(FlaskForm):
    email = StringField(
        "Email", validators=[InputRequired("Please enter your email."), Email()]
    )
    old_password = PasswordField(
        "Current Password",
        validators=[InputRequired("Please enter your current password.")],
    )
    password = PasswordField("Password", validators=[Optional()])
    confirm_password = PasswordField(
        "Confirm Password",
        validators=[Optional(), EqualTo("password", "Please enter the same password.")],
    )
    submit = SubmitField("Update Login Information")

    def validate_old_password(self, field):
        if not current_user.check_password(field.data):
            raise ValidationError("Old password doesn't match.")


class LoginForm(FlaskForm):
    username = StringField(
        "Username",
        validators=[
            InputRequired("Please enter your username."),
            UsernameLengthValidator,
        ],
    )
    password = PasswordField(
        "Password", validators=[InputRequired("Please enter your password.")]
    )
    code = IntegerField("Two-Factor Token", validators=[Optional()])
    remember = BooleanField("Remember Me")
    submit = SubmitField("Login")

    def get_user(self):
        query = User.query.filter(
            func.lower(User.username) == self.username.data.lower()
        )
        return query.first()

    def validate_username(self, field):
        if self.get_user() is None:
            raise ValidationError("This user doesn't exist.")

    def validate_password(self, field):
        user = self.get_user()
        if not user:
            return
        if not user.check_password(field.data):
            raise ValidationError("Check your password again.")


class ProfileEditForm(FlaskForm):
    name = StringField("Name", validators=[InputRequired("Please enter a name.")])
    avatar = FileField("Avatar")
    remove_avatar = BooleanField("Remove Avatar")
    submit = SubmitField("Update Profile")


class PasswordForgotForm(FlaskForm):
    email = StringField(
        "Email",
        validators=[
            InputRequired("Please enter your email."),
            Email("Please enter a valid email."),
        ],
    )
    submit = SubmitField("Send Recovery Email")

    def __init__(self):
        super(PasswordForgotForm, self).__init__()
        self._user = None
        self._user_cached = False

    @property
    def user(self):
        if not self._user_cached:
            self._user = User.query.filter(
                func.lower(User.email) == self.email.data.lower()
            ).first()
            self._user_cached = True
        return self._user


class PasswordResetForm(FlaskForm):
    password = PasswordField(
        "Password", validators=[InputRequired("Please enter a password.")]
    )
    confirm_password = PasswordField(
        "Confirm Password",
        validators=[
            InputRequired("Please confirm your password."),
            EqualTo("password", "Please enter the same password."),
        ],
    )
    submit = SubmitField("Change Password")


class RegisterForm(FlaskForm):
    name = StringField("Name", validators=[InputRequired("Please enter a name.")])
    username = StringField(
        "Username",
        validators=[InputRequired("Please enter a username."), UsernameLengthValidator],
    )
    email = StringField(
        "Email",
        validators=[
            InputRequired("Please enter an email."),
            Email("Please enter a valid email."),
        ],
    )
    password = PasswordField(
        "Password", validators=[InputRequired("Please enter a password.")]
    )
    confirm_password = PasswordField(
        "Confirm Password",
        validators=[
            InputRequired("Please confirm your password."),
            EqualTo("password", "Please enter the same password."),
        ],
    )
    level = RadioField(
        "Who are you?", choices=[("1", "Student"), ("2", "Observer"), ("3", "Teacher")]
    )
    submit = SubmitField("Register")

    def validate_username(self, field):
        if not VALID_USERNAME.match(field.data):
            raise ValidationError(
                "Username must be contain letters, numbers, or _, and not start with a number."
            )
        if User.query.filter(func.lower(User.username) == field.data.lower()).count():
            raise ValidationError("Username is taken.")

    def validate_email(self, field):
        if User.query.filter(func.lower(User.email) == field.data.lower()).count():
            raise ValidationError("Email is taken.")


class TwoFactorAuthSetupForm(FlaskForm):
    code = IntegerField(
        "Code",
        validators=[InputRequired("Please enter the code.")],
        widget=NumberInput(),
    )
    password = PasswordField(
        "Password", validators=[InputRequired("Please enter your password.")]
    )
    submit = SubmitField("Confirm")

    def validate_code(self, field):
        if not current_user.verify_totp(field.data):
            raise ValidationError("Incorrect code.")

    def validate_password(self, field):
        if not current_user.check_password(field.data):
            raise ValidationError("Incorrect password.")
