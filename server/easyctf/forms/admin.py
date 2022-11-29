import imp
import time
from datetime import datetime

from flask_wtf import FlaskForm
from sqlalchemy import and_
from wtforms import ValidationError
from wtforms.fields import (BooleanField, FloatField, HiddenField,
                            IntegerField, StringField, SubmitField,
                            TextAreaField, DateTimeLocalField)
from wtforms.validators import InputRequired, NumberRange, Optional

from easyctf.models import Problem
from easyctf.utils import VALID_PROBLEM_NAME, generate_string


class DateTimeField(DateTimeLocalField):
    def _value(self):
        if not self.data:
            return ""
        return datetime.fromtimestamp(float(self.data)).strftime("%Y-%m-%dT%H:%M")

    def process_formdata(self, valuelist):
        value = valuelist[0]
        obj = datetime.strptime(value, "%Y-%m-%dT%H:%M")
        self.data = time.mktime(obj.timetuple())


class ProblemForm(FlaskForm):
    author = StringField("Problem Author", validators=[InputRequired("Please enter the author.")])
    title = StringField("Problem Title", validators=[InputRequired("Please enter a problem title.")])
    name = StringField("Problem Name (slug)", validators=[InputRequired("Please enter a problem name.")])
    category = StringField("Problem Category", validators=[InputRequired("Please enter a problem category.")])
    description = TextAreaField("Description", validators=[InputRequired("Please enter a description.")])
    value = IntegerField("Value", validators=[InputRequired("Please enter a value.")])
    programming = BooleanField(default=False, validators=[Optional()])

    autogen = BooleanField("Autogen", validators=[Optional()])
    grader = TextAreaField("Grader", validators=[InputRequired("Please enter a grader.")])
    generator = TextAreaField("Generator", validators=[Optional()])
    source_verifier = TextAreaField("Source Verifier", validators=[Optional()])

    test_cases = IntegerField("Test Cases", validators=[Optional()])
    time_limit = FloatField("Time Limit", validators=[Optional()])
    memory_limit = IntegerField("Memory Limit", validators=[Optional()])

    submit = SubmitField("Submit")

    def validate_name(self, field):
        if not VALID_PROBLEM_NAME.match(field.data):
            raise ValidationError("Problem name must be an all-lowercase, slug-style string.")
        # if Problem.query.filter(Problem.name == field.data).count():
        #     raise ValidationError("That problem name already exists.")

    def validate_grader(self, field):
        grader = imp.new_module("grader")
        if self.programming.data:
            # TODO validation
            pass
        else:
            try:
                exec(field.data, grader.__dict__)
                assert hasattr(grader, "grade"), \
                    "Grader is missing a 'grade' function."
                if self.autogen.data:
                    assert hasattr(grader, "generate"), "Grader is missing a 'generate' function."
                    seed1 = generate_string()
                    import random
                    random.seed(seed1)
                    data = grader.generate(random)
                    assert type(data) is dict, "'generate' must return dict"
                else:
                    result = grader.grade(None, "")
                    assert type(result) is tuple, "'grade' must return (correct, message)"
                    correct, message = result
                    assert type(correct) is bool, "'correct' must be a boolean."
                    assert type(message) is str, "'message' must be a string."
            except Exception as e:
                raise ValidationError("%s: %s" % (e.__class__.__name__, str(e)))


class SettingsForm(FlaskForm):
    team_size = IntegerField("Team Size", default=5, validators=[NumberRange(min=1), InputRequired("Please enter a max team size.")])
    ctf_name = StringField("CTF Name", default="OpenCTF", validators=[InputRequired("Please enter a CTF name.")])
    start_time = DateTimeField("Start Time", validators=[InputRequired("Please enter a CTF start time.")])
    end_time = DateTimeField("End Time", validators=[InputRequired("Please enter a CTF end time.")])
    judge_api_key = StringField("Judge API Key", validators=[Optional()])

    submit = SubmitField("Save Settings")

    def validate_start_time(self, field):
        import logging
        logging.error("lol {}".format(field.data))
