from sqlalchemy.engine import reflection
from sqlalchemy.schema import (MetaData, Table, DropTable, ForeignKeyConstraint, DropConstraint)

import pytest

from api import api as ctf_api
from api.app import app as ctf_app
from api.config import options
from api.api.models import db as ctf_db

def db_DropEverything(db):
	# From http://www.sqlalchemy.org/trac/wiki/UsageRecipes/DropEverything

	conn=db.engine.connect()

	# the transaction only applies if the DB supports
	# transactional DDL, i.e. Postgresql, MS SQL Server
	trans = conn.begin()

	inspector = reflection.Inspector.from_engine(db.engine)

	# gather all data first before dropping anything.
	# some DBs lock after things have been dropped in 
	# a transaction.
	metadata = MetaData()

	tbs = []
	all_fks = []

	for table_name in inspector.get_table_names():
		fks = []
		for fk in inspector.get_foreign_keys(table_name):
			if not fk['name']:
				continue
			fks.append(
				ForeignKeyConstraint((),(),name=fk['name'])
				)
		t = Table(table_name,metadata,*fks)
		tbs.append(t)
		all_fks.extend(fks)

	for fkc in all_fks:
		conn.execute(DropConstraint(fkc))

	for table in tbs:
		conn.execute(DropTable(table))

	trans.commit()

@pytest.fixture(scope="session")
def app(request):
	app = ctf_app
	app.config.from_object(options)
	app.config["TESTING"] = True

	ctx = app.test_request_context()
	ctx.push()

	def teardown():
		ctx.pop()

	request.addfinalizer(teardown)
	return app

@pytest.fixture(scope="session")
def client(app):
	return app.test_client()

@pytest.fixture(scope="class")
def db(request, app):
	ctf_db.reflect()
	db_DropEverything(ctf_db)
	ctf_db.create_all()

	def teardown():
		ctf_db.session.close_all()
		ctf_db.reflect()
		db_DropEverything(ctf_db)

	request.addfinalizer(teardown)
	return ctf_db

@pytest.fixture(scope="class")
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