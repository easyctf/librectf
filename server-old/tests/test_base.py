from flask import url_for


class TestBase:
    def test_sanity(self, testapp):
        res = testapp.get("/")
        assert res.status_code == 200
