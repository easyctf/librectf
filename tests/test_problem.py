import json
import pytest

USER = ["Test", "test", "test@test.com", "password", 1, False]
ADMIN = ["Admin", "admin", "admin@admin.com", "password", 1, True]

problem = {
	"title": "Test Problem",
	"category": "Misc",
	"description": "Here is a description.",
	"hint": "No hint.",
	"value": 200,
	"grader_contents": "flag = 'hello'\ndef grade(autogen, candidate):\n\tif candidate == flag:\n\t\treturn True, 'Nice!'\n\treturn False, 'Nope.'",
	"bonus": 0,
	"autogen": 0
}

@pytest.mark.usefixtures("app", "db")
class TestProblem():
	@classmethod
	@pytest.fixture(scope="class", autouse=True)
	def setup_class(cls, app, db):
		app.api.user.register_user(*USER)
		app.api.user.register_user(*ADMIN)

	def test_add_problem(self, client):
		client.post("/api/user/login", data={ "username": ADMIN[1], "password": ADMIN[3] })
		response = json.loads(client.post("/api/problem/add", data=problem).data)
		assert response["success"] == 1