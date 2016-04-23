import os

secret = open(".secret_key", "a+b")
contents = secret.read()
if not contents:
    key = os.urandom(128)
    secret.write(key)
    secret.flush()
else:
    key = contents
secret.close()

SECRET_KEY = key

SQLALCHEMY_DATABASE_URI = "mysql://user:pass@host:port/db"
SQLALCHEMY_TRACK_MODIFICATIONS = False

UPLOAD_FOLDER = os.path.normpath("../web/files")

MAILGUN_URL = ""
MAILGUN_KEY = ""
ADMIN_EMAIL = ""

GRADER_FOLDER = os.path.normpath("graders")

PROBLEM_DIR = "../problems"