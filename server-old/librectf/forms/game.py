import json

from flask_wtf import FlaskForm
from wtforms import ValidationError
from wtforms.fields import StringField
from wtforms.validators import Length


class GameStateUpdateForm(FlaskForm):
    state = StringField("state", validators=[Length(max=4096)])

    def validate_state(self, field):
        try:
            json.loads(field.data)
        except:
            raise ValidationError("invalid json!")
