import json
import pytest

USER = ["Test", "test", "test@test.com", "password", 1, False]
ADMIN = ["Admin", "admin", "admin@admin.com", "password", 1, True]

@pytest.mark.usefixtures("app", "db")
class TestDecorators():
	@classmethod
	@pytest.fixture(scope="class", autouse=True)
	def setup_class(cls, app, db):
		app.api.user.register_user(*USER)
		app.api.user.register_user(*ADMIN)

	def test_api_wrapper(self, app):
		@app.api.decorators.api_wrapper
		def test_function(value):
			if value == True:
				return { "success": 1, "message": "Nice!" }
			else:
				raise app.api.decorators.WebException("Darn.")
		
		true_value = test_function(True).data
		assert type(true_value) == type("string") and json.loads(true_value)["success"] == 1

		false_value = test_function(False).data
		assert type(true_value) == type("string") and json.loads(false_value)["success"] == 0

	def test_login_required(self, app, db):
		@app.api.decorators.login_required
		def protected_function():
			return { "success": 1 }

		func_value = protected_function()
		assert func_value["success"] == 0

		app.api.user.login_user(USER[1], USER[3])
		func_value = protected_function()
		print func_value
		assert func_value["success"] == 1

	def test_admins_only(self, app, db):
		@app.api.decorators.login_required
		@app.api.decorators.admins_only
		def protected_function():
			return { "success": 1 }

		app.api.user.login_user(USER[1], USER[3])
		func_value = protected_function()
		assert func_value["success"] == 0

		app.api.user.logout_user()
		app.api.user.login_user(ADMIN[1], ADMIN[3])
		func_value = protected_function()
		assert func_value["success"] == 1