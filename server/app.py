#!/usr/bin/python

from argparse import ArgumentParser
from flask import Flask, request, send_file, redirect, abort

app = Flask(__name__)

import api
import config
import json
import logging
import os
import traceback

from api.decorators import api_wrapper

app.config.from_object(config.options)

if not os.path.exists(app.config["UPLOAD_FOLDER"]):
    os.makedirs(app.config["UPLOAD_FOLDER"])
if not os.path.exists("pfp"):
	os.makedirs("pfp")

with app.app_context():
	from api.models import db, Files, Teams, Problems, Solves, Users
	db.init_app(app)
	db.create_all()

	app.db = db

app.secret_key = config.SECRET_KEY

app.register_blueprint(api.activity.blueprint, url_prefix="/api/activity")
app.register_blueprint(api.admin.blueprint, url_prefix="/api/admin")
app.register_blueprint(api.problem.blueprint, url_prefix="/api/problem")
app.register_blueprint(api.stats.blueprint, url_prefix="/api/stats")
app.register_blueprint(api.team.blueprint, url_prefix="/api/team")
app.register_blueprint(api.user.blueprint, url_prefix="/api/user")
api.logger.initialize_logs()

@app.route("/api")
@api_wrapper
def api_main():
	return { "success": 1, "message": "The API is online." }

@app.errorhandler(404)
def page_not_found(e):
    return send_file("../web/index.html"), 404

def run(args=None):
	with app.app_context():
		try:
			keyword_args = dict(args._get_kwargs())
			app.debug = keyword_args["debug"] if "debug" in keyword_args else False
		except: pass
		app.run(host="0.0.0.0", port=8000)

def load_problems(args):
	keyword_args = dict(args._get_kwargs())
	force = keyword_args["force"] if "force" in keyword_args else False

	if not os.path.exists(config.options.PROBLEM_DIR):
		api.logger.log("api.problem.log", "Problems directory doesn't exist.")
		return

	for (dirpath, dirnames, filenames) in os.walk(config.options.PROBLEM_DIR):
		if "problem.json" in filenames:
			json_file = os.path.join(dirpath, "problem.json")
			contents = open(json_file).read()
			try:
				data = json.loads(contents)
			except ValueError as e:
				api.logger.log("api.problem.log", "Invalid JSON format in file {filename} ({exception})".format(filename=json_file, exception=e))
				continue
			if not isinstance(data, dict):
				api.logger.log("api.problem.log", "{filename} is not a dict.".format(filename=json_file))
				continue

			missing_keys = []
			for key in ["pid", "title", "category", "value"]:
				if key not in data:
					missing_keys.append(key)
			if len(missing_keys) > 0:
				api.logger.log("api.problem.log", "{filename} is missing the following keys: {keys}".format(filename=json_file, keys=", ".join(missing_keys)))
				continue

			relative_path = os.path.relpath(dirpath, config.options.PROBLEM_DIR)
			data["description"] = open(os.path.join(dirpath, "description.md"), "r").read()
			api.logger.log("api.problem.log", "Found problem '{}'".format(data["title"]))
			with app.app_context():
				try:
					api.problem.insert_problem(data, force=force)
				except Exception as e:
					api.logger.log("api.problem.log", "Problem '{}' was not added to the database. Error: {}".format(data["title"], e))
					api.logger.log("api.problem.log", "{}".format(traceback.format_exc()))

	api.logger.log("api.problem.log", "Finished.")

def main():
	parser = ArgumentParser(description="OpenCTF Server Management")

	subparser = parser.add_subparsers(help="Select one of the following actions.")
	parser_problems = subparser.add_parser("problems", help="Manage problems.")
	subparser_problems = parser_problems.add_subparsers(help="Select one of the following actions.")
	parser_problems_load = subparser_problems.add_parser("load", help="Load all problems into database.")
	parser_problems_load.add_argument("-f", "--force", action="store_true", help="Force overwrite problems.", default=False)
	parser_problems_load.set_defaults(func=load_problems)

	parser_run = subparser.add_parser("run", help="Run the server.")
	parser_run.add_argument("-d", "--debug", action="store_true", help="Run the server in debug mode.", default=False)
	parser_run.set_defaults(func=run)

	args = parser.parse_args()

	if "func" in args:
		args.func(args)
	else:
		parser.print_help()

if __name__ == "__main__":
	main()
