from PIL import Image

import json
import pytest
import StringIO

USER = ["Test", "test", "test@test.com", "password", 1, False]
login_data = { "username": USER[1], "password": USER[3] }

@pytest.mark.usefixtures("app", "db")
class TestUser():
	def test_user_registration(self, client, app, db):
		data = {
			"name": USER[0],
			"username": USER[1],
			"email": USER[2],
			"password": USER[3],
			"password_confirm": USER[3],
			"type": USER[4]
		}
		response = json.loads(client.post("/api/user/register", data=data).data)
		assert response["success"] == 1
		assert db.session.query(app.api.models.Users).filter_by(username=USER[1]).first() is not None

	def test_user_login(self, client):
		response = json.loads(client.post("/api/user/login", data=login_data).data)
		assert response["success"] == 1
		response = json.loads(client.get("/api/user/status").data)
		assert response["logged_in"] == True

	def test_user_logout(self, client):
		client.post("/api/user/login", data=login_data)
		client.get("/api/user/logout")
		response = json.loads(client.get("/api/user/status").data)
		assert response["logged_in"] == False

	def test_user_status(self, client):
		response = json.loads(client.get("/api/user/status").data)
		required_keys = ["username", "competition", "in_team", "logged_in", "ctf_name"]
		assert response["success"] == 1
		assert all([key in response.keys() for key in required_keys])

	def test_user_info(self, client):
		response = json.loads(client.get("/api/user/info?username=%s" % USER[1]).data)
		assert response["success"] == 1

	def test_user_info_without_username(self, client):
		response = json.loads(client.get("/api/user/info").data)
		assert response["success"] == 0

	def test_user_info_without_username_but_logged_in(self, client):
		client.post("/api/user/login", data=login_data)
		response = json.loads(client.get("/api/user/info").data)
		assert response["success"] == 1

	def test_get_avatar(self, client):
		avatar = client.get("/api/user/avatar/%s" % USER[1]).data
		buff = StringIO.StringIO()
		buff.write(avatar)
		im = Image.open(buff)
		assert im.size == (256, 256)

	def test_upload_avatar(self, client):
		client.post("/api/user/login", data=login_data)
		data = {
			"file": (self.white_image(), "some_random_filename.png")
		}
		response = json.loads(client.post("/api/user/avatar/upload", content_type="multipart/form-data", data=data).data)
		assert response["success"] == 1

	def test_remove_avatar(self, client):
		client.post("/api/user/login", data=login_data)
		response = json.loads(client.post("/api/user/avatar/remove").data)
		assert response["success"] == 1

	def white_image(self):
		new_avatar = Image.new("RGB", (300, 100), "white")
		stringIO = StringIO.StringIO()
		new_avatar.save(stringIO, format="PNG")
		stringIO.seek(0)
		return stringIO