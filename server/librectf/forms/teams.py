from flask_login import current_user
from flask_wtf import FlaskForm
from sqlalchemy import func, and_
from wtforms import ValidationError
from wtforms.fields import BooleanField, FileField, StringField, SubmitField
from wtforms.validators import InputRequired, Length

from easyctf.forms.validators import TeamLengthValidator
from easyctf.models import Config, Team, User


class AddMemberForm(FlaskForm):
    username = StringField(
        "Username",
        validators=[
            InputRequired(
                "Please enter the username of the person you would like to add."
            )
        ],
    )
    submit = SubmitField("Add")

    def get_user(self):
        query = User.query.filter(
            func.lower(User.username) == self.username.data.lower()
        )
        return query.first()

    def validate_username(self, field):
        if not current_user.team:
            raise ValidationError("You must belong to a team.")
        if current_user.team.owner != current_user.uid:
            raise ValidationError("Only the team captain can invite new members.")
        if len(current_user.team.outgoing_invitations) >= Config.get_team_size():
            raise ValidationError(
                "You've already sent the maximum number of invitations."
            )
        user = User.query.filter(
            func.lower(User.username) == field.data.lower()
        ).first()
        if user is None:
            raise ValidationError("This user doesn't exist.")
        if user.tid is not None:
            raise ValidationError("This user is already a part of a team.")
        if user in current_user.team.outgoing_invitations:
            raise ValidationError("You've already invited this member.")


class CreateTeamForm(FlaskForm):
    teamname = StringField(
        "Team Name",
        validators=[InputRequired("Please create a team name."), TeamLengthValidator],
    )
    school = StringField(
        "School",
        validators=[
            InputRequired("Please enter your school."),
            Length(
                3,
                36,
                "Your school name must be between 3 and 36 characters long. Use abbreviations if necessary.",
            ),
        ],
    )
    submit = SubmitField("Create Team")

    def validate_teamname(self, field):
        if current_user.tid is not None:
            raise ValidationError("You are already in a team.")
        if Team.query.filter(func.lower(Team.teamname) == field.data.lower()).count():
            raise ValidationError("Team name is taken.")


class DisbandTeamForm(FlaskForm):
    teamname = StringField("Confirm Team Name")
    submit = SubmitField("Delete Team")

    def validate_teamname(self, field):
        if not current_user.team:
            raise ValidationError("You must belong to a team.")
        if current_user.team.owner != current_user.uid:
            raise ValidationError("Only the team captain can disband the team.")
        if field.data != current_user.team.teamname:
            raise ValidationError("Incorrect confirmation.")


class ManageTeamForm(FlaskForm):
    teamname = StringField(
        "Team Name",
        validators=[InputRequired("Please create a team name."), TeamLengthValidator],
    )
    school = StringField(
        "School",
        validators=[
            InputRequired("Please enter your school."),
            Length(
                3,
                36,
                "Your school name must be between 3 and 36 characters long. Use abbreviations if necessary.",
            ),
        ],
    )
    submit = SubmitField("Update")

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.tid = kwargs.get("tid", None)

    def validate_teamname(self, field):
        if Team.query.filter(
            and_(func.lower(Team.teamname) == field.data.lower(), Team.tid != self.tid)
        ).count():
            raise ValidationError("Team name is taken.")


class ProfileEditForm(FlaskForm):
    teamname = StringField(
        "Team Name",
        validators=[InputRequired("Please enter a team name."), TeamLengthValidator],
    )
    school = StringField(
        "School",
        validators=[
            InputRequired("Please enter your school."),
            Length(
                3,
                36,
                "Your school name must be between 3 and 36 characters long. Use abbreviations if necessary.",
            ),
        ],
    )
    avatar = FileField("Avatar")
    remove_avatar = BooleanField("Remove Avatar")
    submit = SubmitField("Update Profile")
