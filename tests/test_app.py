from flask.ext.testing import TestCase

from server.app import app
from server.config import options

class AppTestCase(TestCase):
	def create_app(self):
		app.config['SQLALCHEMY_DATABASE_URI'] = options.SQLALCHEMY_TEST_DATABASE_URI
		app.config['TESTING'] = True
		return app

	def setUp(self):
		pass

	def tearDown(self):
		pass

	def test_api_is_up_and_running(self):
		response = self.client.get("/api").json
		response = dict([(str(k), v) for k, v in response.items()])
		self.assertEquals(response, dict(success=1, message="The API is online."))

	def test_404(self):
		response = self.client.get("/four-oh-four")
		self.assert404(response)
