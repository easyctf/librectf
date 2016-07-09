import json
import pytest

@pytest.mark.usefixtures("db")
class TestGeneral():
	def test_api_is_running(self, client):
		response = json.loads(client.get("/api").data)
		assert response["success"] == 1

	def test_404(self, client):
		response = json.loads(client.get("/404").data)
		print response["success"] == 0

	def test_requires_setup(self, client):
		response = json.loads(client.get("/api/user/status").data)
		assert "redirect" in response and response["redirect"] == "/setup"

	def test_setup(self, app, client):
		prepare = json.loads(client.get("/api/admin/setup/init").data)
		assert prepare["success"] == 1

		options = {
			"name": "Test",
			"email": "test@test.com",
			"username": "admin",
			"password": "password",
			"password_confirm": "password",
			"type": "1",
			"verification": app.api.utils.get_config("setup_verification")
		}
		response = json.loads(client.post("/api/admin/setup", data=options).data)
		print response
		assert response["success"] == 1

	def test_setup_complete(self, client):
		response = json.loads(client.get("/api/user/status").data)
		assert "redirect" not in response