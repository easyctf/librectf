import json
import pytest

USER = ["Test", "test", "test@test.com", "password", 1, False]
ADMIN = ["Admin", "admin", "admin@admin.com", "password", 1, True]


@pytest.mark.usefixtures("app", "db")
class TestProblem():
	@classmethod
	@pytest.fixture(scope="class", autouse=True)
	def setup_class(cls, app, db):
		app.api.user.register_user(*USER)
		app.api.user.register_user(*ADMIN)

	def test_add_problem(self, client):
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
		client.post("/api/user/login", data={ "username": ADMIN[1], "password": ADMIN[3] })
		response = json.loads(client.post("/api/problem/add", data=problem).data)
		assert response["success"] == 1

	def test_get_problem(self, client):
		client.post("/api/user/login", data={ "username": USER[1], "password": USER[3] })
		client.post("/api/team/create", data={ "teamname": "Test", "school": "Test" })
		client.post("/api/team/finalize")
		response = json.loads(client.get("/api/problem/data").data)
		assert response["success"] == 1
		assert "problems" in response and len(response["problems"]) > 0
		assert response["problems"][0]["title"] == "Test Problem"

	def test_add_weighted_problem(self, client):
		problem = {
			"title": "Test Problem 2",
			"category": "Misc",
			"description": "Here is a description.",
			"hint": "No hint.",
			"value": 200,
			"grader_contents": "flag = 'hello'\ndef grade(autogen, candidate):\n\tif candidate == flag:\n\t\treturn True, 'Nice!'\n\treturn False, 'Nope.'",
			"bonus": 0,
			"autogen": 0,
			"weightmap": '{ "test-problem": 1 }',
			"threshold": 1
		}
		client.post("/api/user/login", data={ "username": ADMIN[1], "password": ADMIN[3] })
		response = json.loads(client.post("/api/problem/add", data=problem).data)
		assert response["success"] == 1

	def test_get_weighted_problem_before_solve(self, client):
		client.post("/api/user/login", data={ "username": USER[1], "password": USER[3] })
		response = json.loads(client.get("/api/problem/data").data)
		assert response["success"] == 1
		assert "problems" in response and len(response["problems"]) == 1

	def test_solve_problem(self, client):
		client.post("/api/user/login", data={ "username": USER[1], "password": USER[3] })
		response = json.loads(client.post("/api/problem/submit", data={ "pid": "test-problem", "flag": "hello" }).data)
		assert response["success"] == 1

	def test_get_weighted_problem_after_solve(self, client):
		client.post("/api/user/login", data={ "username": USER[1], "password": USER[3] })
		response = json.loads(client.get("/api/problem/data").data)
		assert response["success"] == 1
		assert "problems" in response and len(response["problems"]) == 2