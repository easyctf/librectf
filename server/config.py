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

CTF_BEGIN = 0 # To be used later
CTF_END = 0 # To be used later

MG_HOST = ""
MG_API_KEY = ""
ADMIN_EMAIL = ""

PROBLEM_DIR = "../problems"