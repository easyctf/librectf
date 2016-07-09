import os

class Object(object):
    def __init__(self, d):
        for a, b in d.items():
            if isinstance(b, (list, tuple)):
               setattr(self, a, [Object(x) if isinstance(x, dict) else x for x in b])
            else:
               setattr(self, a, Object(b) if isinstance(b, dict) else b)

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
ROOT_FOLDER = os.path.abspath(os.path.join(os.path.realpath(__file__), ".."))
DEFAULT_DATABASE_URI = "mysql://root:password@db_1/openctf"

options = {
	"SQLALCHEMY_DATABASE_URI": os.getenv("SQLALCHEMY_DATABASE_URI", DEFAULT_DATABASE_URI),
	"SQLALCHEMY_TEST_DATABASE_URI": os.getenv("SQLALCHEMY_TEST_DATABASE_URI", DEFAULT_DATABASE_URI),
	"SQLALCHEMY_TRACK_MODIFICATIONS": False,
	"ROOT_FOLDER": ROOT_FOLDER,
	"PFP_FOLDER": os.path.abspath(os.path.join(ROOT_FOLDER, "pfp")),
	"UPLOAD_FOLDER": os.path.abspath(os.path.join(ROOT_FOLDER, "files")),
	"MAILGUN_URL": os.getenv("MAILGUN_URL", ""),
	"MAILGUN_KEY": os.getenv("MAILGUN_KEY", ""),
	"ADMIN_EMAIL": os.getenv("ADMIN_EMAIL", "admin@openctf.com"),
	"GRADER_FOLDER": os.path.abspath(os.path.join(ROOT_FOLDER, "graders")),
	"PROBLEM_DIR": os.path.abspath(os.path.join(ROOT_FOLDER, "problems"))
}

# if there's a .env configuration file in the current directory, read it.
# otherwise, use environmental variables
if os.path.exists(".env"):
	data = open(".env", "r")
	for line in data:
		if line.find("=") > 0:
			key = line.split("=")[0]
			value = line.split(key + "=")[1].strip("\n").strip("\r")
			options[key] = value

options = Object(options)
