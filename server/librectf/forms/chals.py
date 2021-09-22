from flask_wtf import FlaskForm
from wtforms import ValidationError
from wtforms.fields import HiddenField, StringField, TextAreaField
from wtforms.validators import InputRequired

from librectf.constants import SUPPORTED_LANGUAGES


class ProblemSubmitForm(FlaskForm):
    pid = HiddenField("Problem ID")
    flag = StringField("Flag", validators=[InputRequired("Please enter a flag.")])


class ProgrammingSubmitForm(FlaskForm):
    pid = HiddenField()
    code = TextAreaField("Code", validators=[InputRequired("Please enter code.")])
    language = HiddenField()

    def validate_language(self, field):
        if field.data not in SUPPORTED_LANGUAGES:
            raise ValidationError("Invalid language.")

    def validate_code(self, field):
        if len(field.data) > 65536:
            raise ValidationError("Code too large! (64KB max)")
