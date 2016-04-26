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

SQLALCHEMY_DATABASE_URI = "mysql://root:i_hate_passwords@localhost/openctf"
SQLALCHEMY_TRACK_MODIFICATIONS = False

ROOT_FOLDER = os.path.abspath(os.path.join(os.path.realpath(__file__), '../..'))

UPLOAD_FOLDER = os.path.abspath(os.path.join(ROOT_FOLDER, "web/files"))

MAILGUN_URL = "https://api.mailgun.net/v3/example.com"
MAILGUN_KEY = "key-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
ADMIN_EMAIL = "EasyCTF Team <team@easyctf.com>"

GRADER_FOLDER = os.path.abspath(os.path.join(ROOT_FOLDER, "server/graders"))

PROBLEM_DIR = os.path.abspath(os.path.join(ROOT_FOLDER, "problems"))
