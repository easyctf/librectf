from flask_wtf import FlaskForm
from sqlalchemy import func
from wtforms import ValidationError
from wtforms.fields import *
from wtforms.validators import *

from easyctf.models import Team


class NewClassroomForm(FlaskForm):
    name = StringField("Classroom Name", validators=[InputRequired()])
    submit = SubmitField("Create")


class AddTeamForm(FlaskForm):
    name = StringField("Team Name", validators=[InputRequired()])
    submit = SubmitField("Add Team")

    def validate_name(self, field):
        if not Team.query.filter(
            func.lower(Team.teamname) == field.data.lower()
        ).count():
            raise ValidationError("Team does not exist!")
