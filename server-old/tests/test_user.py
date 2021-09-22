from flask import url_for

from librectf.models import User

class TestUser:
    def test_user_register(self, testapp, db):
        dispname = "display name"
        email = "fake_email@easyctf.com",
        res = testapp.post(url_for("users.register"), data=dict(
            name=dispname,
            email=email,
            username="user123",
            password="pass456",
            level=1,
        ))
        assert res.status_code == 302

        # make sure the user got added
        user = User.query.filter(email=email).first()
        assert user is not None

        assert user.name == dispname
