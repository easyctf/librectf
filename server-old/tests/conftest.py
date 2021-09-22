import os
import tempfile

import pytest
from webtest import TestApp

from librectf import create_app
from librectf.config import Config
from librectf.objects import db as _db


@pytest.yield_fixture(scope="function")
def app():
    config = Config(testing=True, secret_key="asdf")
    _app = create_app(config)

    with _app.app_context():
        _db.create_all()

    ctx = _app.test_request_context()
    ctx.push()

    yield _app

    ctx.pop()


@pytest.fixture(scope="function")
def testapp(app):
    return TestApp(app)


@pytest.yield_fixture(scope="function")
def db(app):
    _db.app = app
    with app.app_context():
        _db.create_all()

    yield _db

    _db.session.close()
    _db.drop_all()


# @pytest.fixture
# def user(db):
#     """A user for the tests."""
#     class User():
#         def get(self):
#             muser = UserFactory(password='myprecious')
#             UserProfile(muser).save()
#             db.session.commit()
#             return muser
#     return User()
