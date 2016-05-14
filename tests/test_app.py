import unittest
import json

from server.app import app
from server.config import options

class AppTestCase(unittest.TestCase):
	def setUp(self):
		app.config["TESTING"] = True

	def tearDown(self):
		pass

	def test_api_is_up_and_running(self):
		with app.test_client() as client:
			response = json.loads(client.get("/api").data)
			self.assertEquals(response, dict(success=1, message="The API is online."))

	def test_404(self):
		with app.test_client() as client:
			response = client.get("/four-oh-four").status_code
			self.assertEquals(response, 404)
