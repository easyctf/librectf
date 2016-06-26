import pytest

from server.app import app as openctf_app
from server.config import options
from server.api.models import db as app_db


@pytest.fixture(scope='session')
def app(request):
	app = openctf_app
	app.config.from_object(options)
	app.config["TESTING"] = True

	# Establish a request context before running the tests.
	ctx = app.test_request_context()
	ctx.push()

	def teardown():
		ctx.pop()

	request.addfinalizer(teardown)
	return app


@pytest.fixture(scope='session')
def client(app):
	return app.test_client()


@pytest.fixture(scope='class')
def db(request, app):
	app_db.reflect()  # Weird hack
	app_db.drop_all()

	app_db.create_all()

	def teardown():
		app_db.session.close()
		app_db.drop_all()

	request.addfinalizer(teardown)
	return app_db


@pytest.fixture(scope='class')
def session(request, db):
	connection = db.engine.connect()
	transaction = connection.begin()

	options = dict(bind=connection, binds={})
	session = db.create_scoped_session(options=options)

	db.session = session

	def teardown():
		transaction.rollback()
		connection.close()
		session.remove()

	request.addfinalizer(teardown)
	return session
