import unittest
import json

from server.app import app
from server.config import options


class TestThings():
	def test_api_is_up_and_running(self, client):
		response = json.loads(client.get("/api").data)
		assert response == {"success": 1, "message": "The API is online."}

	def test_404(self, client):
		response = client.get("/four-oh-four")
		assert response.status_code == 404
