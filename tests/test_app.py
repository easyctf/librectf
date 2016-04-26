from flask.ext.testing import TestCase

from server.app import app
from server.api.models import db

class AppTestCase(TestCase):
	def create_app(self):
		app.config['SQLALCHEMY_DATABASE_URI'] = 'mysql://root:i_hate_passwords@localhost/openctf'
		app.config['TESTING'] = True

		return app

	def setUp(self):
		db.create_all()

	def tearDown(self):
		db.session.remove()
		db.drop_all()

	def test_api_is_up_and_running(self):
		response = self.client.get("/api").json
		response = dict([(str(k), v) for k, v in response.items()])
		self.assertEquals(response, dict(success=1, message="The API is online."))

	def test_404(self):
		response = self.client.get("/four-oh-four")
		self.assert404(response)

	