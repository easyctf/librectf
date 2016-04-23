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

UPLOAD_FOLDER = os.path.normpath("../web/files")

MAILGUN_URL = "https://api.mailgun.net/v3/easyctf.com"
MAILGUN_KEY = "key-18dc2ef0ebf9c9695fb566c6fe203dc4"
ADMIN_EMAIL = "EasyCTF Team <team@easyctf.com>"

GRADER_FOLDER = os.path.normpath("graders")

PROBLEM_DIR = "../problems"