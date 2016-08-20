import json
import pytest
import re

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

	def test_get_problem(self, app, client):
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

	def test_get_solves(self, client):
		client.post("/api/user/login", data={ "username": USER[1], "password": USER[3] })
		response = json.loads(client.post("/api/problem/solves", data={ "pid": "test-problem" }).data)
		assert response["success"] == 1
		assert "solves" in response and len(response["solves"]) == 1

	def test_clear_submissions(self, client):
		client.post("/api/user/login", data={ "username": ADMIN[1], "password": ADMIN[3] })
		client.post("/api/problem/clear_submissions", data={ "pid": "test-problem" })
		client.post("/api/user/login", data={ "username": USER[1], "password": USER[3] })
		response = json.loads(client.post("/api/problem/solves", data={ "pid": "test-problem" }).data)
		assert response["success"] == 1
		assert "solves" in response and len(response["solves"]) == 0

	def test_add_autogen_problem(self, client):
		problem = {
			"title": "Test Problem 3",
			"category": "Misc",
			"description": "Here is a description.",
			"hint": "No hint.",
			"value": 200,
			"grader_contents": "flag = 'hello'\ndef generate_flag(random):\n\treturn 2 * random.randint(1, 500)\ndef generate_problem(random, pid):\n\treturn {'description': 'what is twice ' + str(random.randint(1, 500)) + '?'}\ndef grade(autogen, candidate):\n\tif int(candidate) == generate_flag(autogen):\n\t\treturn True, 'Nice!'\n\treturn False, 'Nope.'",
			"bonus": 0,
			"autogen": 1,
		}
		client.post("/api/user/login", data={ "username": ADMIN[1], "password": ADMIN[3] })
		response = json.loads(client.post("/api/problem/add", data=problem).data)
		assert response["success"] == 1

	def test_solve_autogen_problem(self, client):
		client.post("/api/user/login", data={ "username": USER[1], "password": USER[3] })
		problems = json.loads(client.get("/api/problem/data").data)["problems"]
		problem = filter(lambda p: p["pid"] == "test-problem-3", problems)[0]
		search = re.search("what is twice (\d+)?", problem["description"])
		number = int(search.group(1))
		assert number >= 1 and number <= 500
		response = json.loads(client.post("/api/problem/submit", data={ "pid": "test-problem-3", "flag": str(2 * number) }).data)
		assert response["success"] == 1