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

ROOT_FOLDER = os.path.abspath(os.path.join(os.path.realpath(__file__), '../..'))

UPLOAD_FOLDER = os.path.abspath(os.path.join(ROOT_FOLDER, "web/files"))

MAILGUN_URL = ""
MAILGUN_KEY = ""
ADMIN_EMAIL = ""

GRADER_FOLDER = os.path.abspath(os.path.join(ROOT_FOLDER, "server/graders"))

PROBLEM_DIR = os.path.abspath(os.path.join(ROOT_FOLDER, "problems"))
