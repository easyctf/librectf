from flask import Blueprint, jsonify, request
from flask import current_app as app
from decorators import admins_only, api_wrapper, webhook_wrapper, WebException
from models import db, Config, Problems, Teams, Users, UserActivity
from schemas import verify_to_schema, check
from operator import itemgetter
from StringIO import StringIO

import git
import hashlib
import hmac
import json
import logger
import markdown2
import os
import paramiko
import problem
import shutil
import team
import threading
import user
import utils
import yaml

blueprint = Blueprint("admin", __name__)
SSH_FOLDER = os.path.expanduser("~/.ssh")
if not os.path.exists(SSH_FOLDER):
	os.mkdir(SSH_FOLDER)
SSH_CONFIG_FILE = os.path.join(SSH_FOLDER, "config")
if not os.path.exists(SSH_CONFIG_FILE):
	os.mknod(SSH_CONFIG_FILE)
GIT_DIR = os.path.expanduser("~/git")
if not os.path.exists(GIT_DIR):
	os.mkdir(GIT_DIR)
KEYFILE = os.path.expanduser("~/key")

@blueprint.route("/setup/init")
@api_wrapper
def admin_setup_init():
	if utils.is_setup_complete(): raise WebException("Setup has already been complete.")

	verification = Config("setup_verification", utils.generate_string().lower())
	with app.app_context():
		for item in Config.query.filter_by(key="setup_verification").all():
			db.session.delete(item)
		db.session.add(verification)
		db.session.commit()

		db.session.close()
	return { "success": 1 }

@blueprint.route("/setup", methods=["POST"])
@api_wrapper
def admin_setup():
	global user
	params = utils.flat_multi(request.form)
	if utils.is_setup_complete(): raise WebException("Setup has already been complete.")

	if params.get("verification") != Config.query.filter_by(key="setup_verification").first().value:
		raise WebException("Verification does not match.")

	if params.get("password") != params.get("password_confirm"):
		raise WebException("Passwords do not match.")
	verify_to_schema(user.UserSchema, params)

	name = params.get("name")
	email = params.get("email")
	username = params.get("username")
	password = params.get("password")
	password_confirm = params.get("password_confirm")
	utype = int(params.get("type"))

	setup_vars = [
		Config("ctf_name", params.get("ctf_name")),
		Config("start_time", params.get("start_time")),
		Config("end_time", params.get("end_time")),
		Config("team_size", params.get("team_size")),
		Config("stylesheet", "https://maxcdn.bootstrapcdn.com/bootstrap/3.3.6/css/bootstrap.min.css"),
		Config("setup_complete", True)
	]

	#  _user = Users(name, username, email, password, utype=utype, admin=True)
	user.register_user(name, username, email, password, utype=utype, admin=True)
	with app.app_context():
		for var in setup_vars:
			db.session.add(var)
			db.session.commit()
			db.session.close()

	logger.log(__name__, "%s registered with %s" % (name.encode("utf-8"), email.encode("utf-8")))
	user.login_user(username, password)

	return { "success": 1, "message": "Success!" }

@blueprint.route("/stats/overview")
@api_wrapper
@admins_only
def admin_stats_overview():
	overview = { }
	overview["num_users"] = user.num_users(), user.num_users(include_observers=True)
	overview["num_teams"] = team.num_teams(), team.num_teams(include_observers=True)
	overview["num_problems"] = problem.num_problems()

	return { "success": 1, "overview": overview }

@blueprint.route("/settings")
@api_wrapper
@admins_only
def admin_settings():
	return { "success": 1, "settings": get_settings() }

@blueprint.route("/settings/update", methods=["POST"])
@api_wrapper
@admins_only
def admin_settings_update():
	params = utils.flat_multi(request.form)
	params.pop("csrf_token")
	with app.app_context():
		for key in params:
			config = Config.query.filter_by(key=key).first()
			if config is None:
				config = Config(key, params[key])
			else:
				new = params[key]
				if config.value != new:
					config.value = params[key]
			db.session.add(config)
		db.session.commit()

	return { "success": 1, "message": "Success!" }

@blueprint.route("/teams/overview")
@api_wrapper
@admins_only
def admin_team_overview():
	teams_return = []
	teams = Teams.query.all()
	for _team in teams:
		teams_return.append(_team.get_info())
	teams_return.sort(key=itemgetter("points"), reverse=True)
	return { "success": 1, "teams": teams_return }

@blueprint.route("/info")
@api_wrapper
def admin_info():
	settings = get_settings()
	result = { }
	if "start_time" in settings: result["start_time"] = settings["start_time"]
	if "end_time" in settings: result["end_time"] = settings["end_time"]
	if "team_size" in settings: result["team_size"] = settings["team_size"]
	if "stylesheet" in settings: result["stylesheet"] = settings["stylesheet"]
	return { "success": 1, "info": result }

@blueprint.route("/webhook", methods=["POST"])
@webhook_wrapper
def github_webhook():
	secret = utils.get_config("webhook_secret", "")
	if len(secret) == 0:
		raise WebException("A webhook has not been enabled for this platform. Set a secret to enable the webhook.")
	payload = request.get_data()
	hashed = hmac.new(secret, payload, hashlib.sha1)
	if request.headers["X-Hub-Signature"] != "sha1=%s" % hashed.hexdigest():
		raise WebException("Forged request detected.")
	data = json.loads(payload)
	url = data["repository"]["ssh_url"]
	key, dummy = utils.get_ssh_keys()
	client = paramiko.SSHClient()
	client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
	private_key = paramiko.RSAKey.from_private_key(StringIO(key))
	try:
		client.connect(hostname="github.com", username="git", pkey=private_key)
	except paramiko.AuthenticationException:
		raise WebException("Github is denying access to this repository. Make sure the public key has been installed correctly.")
	data["delivery_id"] = request.headers["X-GitHub-Delivery"]
	# thread = threading.Thread(target=import_repository, args=(data))
	clone_repository(data)
	return { "success": 1, "message": "Initiated import." }

def clone_repository(payload):
	GIT_REPO = str(os.path.join(GIT_DIR, payload["delivery_id"]))
	if os.path.exists(GIT_REPO):
		shutil.rmtree(GIT_REPO)
	repo = git.Repo.init(GIT_REPO)
	origin = repo.create_remote("origin", payload["repository"]["ssh_url"])
	with open(KEYFILE, "w") as f:
		f.write(utils.get_ssh_keys()[0])
	with open(SSH_CONFIG_FILE, "w") as f:
		f.write("Host *\n\tStrictHostKeyChecking no")
	os.chmod(KEYFILE, 0600)
	os.chmod(SSH_CONFIG_FILE, 0600)
	if os.system("cd %s; ssh-agent bash -c 'ssh-add %s; git pull origin master'" % (GIT_REPO, KEYFILE)) == 0:
		os.unlink(KEYFILE)
		problems = []
		for problem in os.listdir(GIT_REPO):
			problem = str(problem)
			if problem in [".", "..", ".git", ".exclude"]: continue
			if not os.path.isdir(os.path.join(GIT_REPO, problem)): continue
			files = os.listdir(os.path.join(GIT_REPO, problem))
			for required_file in ["grader.py", "problem.yml", "description.md"]:
				if required_file not in files:
					raise WebException("Expected required file %s in '%s'." % (required_file, problem))
			metadata = yaml.load(open(os.path.join(GIT_REPO, problem, "problem.yml")))
			for required_key in ["title", "category", "value"]:
				if required_key not in metadata:
					raise WebException("Expected required key %s in 'problem.yml'." % required_key)
			problems.append(problem)
		# thread = threading.Thread(target=import_repository, args=(GIT_REPO, problems))
		import_repository(GIT_REPO, problems)
	else:
		raise WebException("Failed to pull from remote.")

def import_repository(path, problems):
	logger.log(__name__, "Importing %s" % str(problems))
	for problem in problems:
		problem_path = os.path.join(path, problem)
		if os.path.isdir(problem_path):
			import_problem(problem_path, problem)

def import_problem(path, pid):
	with app.app_context():
		existing_problem = Problems.query.filter_by(pid=pid).first()
		if existing_problem is not None:
			db.session.delete(existing_problem)
			db.session.commit()
		metadata = yaml.load(open(os.path.join(path, "problem.yml")))
		title = metadata.get("title")
		category = metadata.get("category")
		value = int(metadata.get("value"))
		hint = metadata.get("hint")
		description = open(os.path.join(path, "description.md")).read()
		grader = open(os.path.join(path, "grader.py")).read()

		if "files" in metadata:
			files = metadata["files"]
			files_dir = os.path.join(app.config["UPLOAD_FOLDER"], pid)
			if os.path.exists(files_dir):
				shutil.rmtree(files_dir)
			os.mkdir(files_dir)
			for file in files:
				src = os.path.join(path, file)
				if os.path.exists(src):
					shutil.copyfile(src, os.path.join(files_dir, file))

		try:
			problem.add_problem(title, category, description, value, grader, pid=pid, hint=hint)
		except Exception, e:
			logger.log(__name__, "Error when importing problem '%s': %s" % (pid, str(e)))

def get_settings():
	settings_return = {}
	settings = Config.query.all()
	for setting in settings:
		if setting.key in ["public_key", "private_key"]: continue
		settings_return[setting.key] = setting.value
		if setting.key == "webhook_secret" and len(setting.value) > 1:
			dummy, settings_return["public_key"] = utils.get_ssh_keys()
	return settings_return
