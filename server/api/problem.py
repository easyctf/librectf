import hashlib
import json
import logger
import os
import shutil
import utils

from flask import Blueprint, jsonify, session, request
from flask import current_app as app
from werkzeug import secure_filename

from models import db, Files, Problems, Solves, Teams
from decorators import admins_only, api_wrapper, login_required, InternalException, WebException

blueprint = Blueprint("problem", __name__)

@blueprint.route("/add", methods=["POST"])
@admins_only
@api_wrapper
def problem_add():
	title = request.form["title"]
	category = request.form["category"]
	description = request.form["description"]
	hint = request.form["hint"]
	value = request.form["value"]
	grader_contents = request.form["grader_contents"]
	bonus = request.form["bonus"]

	title_exists = Problems.query.filter_by(title=title).first()
	if title_exists:
		raise WebException("Problem name already taken.")

	try:
		exec(grader_contents)
	except Exception, e:
		raise WebException("There is a syntax error in the grader: %s" % e)

	pid = utils.generate_string()
	while Problems.query.filter_by(pid=pid).first():
		pid = utils.generate_string()

	grader_folder = os.path.join(app.config["GRADER_FOLDER"], pid)
	if not os.path.exists(grader_folder):
		os.makedirs(grader_folder)
	grader_path = os.path.join(grader_folder, "grader.py")
	grader_file = open(grader_path, "w")
	grader_file.write(grader_contents)
	grader_file.close()

	problem = Problems(pid, title, category, description, value, grader_path, bonus=bonus, hint=hint)
	db.session.add(problem)

	files = request.files.getlist("files[]")
	for _file in files:
		filename = secure_filename(_file.filename)

		if len(filename) == 0:
			continue

		file_path = os.path.join(app.config["UPLOAD_FOLDER"], filename)

		_file.save(file_path)
		db_file = Files(problem.pid, "/".join(file_path.split("/")[2:]))
		db.session.add(db_file)

	db.session.commit()

	return { "success": 1, "message": "Success!" }

@blueprint.route("/delete", methods=["POST"])
@admins_only
@api_wrapper
def problem_delete():
	pid = request.form["pid"]
	problem = Problems.query.filter_by(pid=pid).first()
	if problem:
		Solves.query.filter_by(pid=pid).delete()
		Problems.query.filter_by(pid=pid).delete()
		grader_folder = "/".join(problem.grader.split("/")[:-1])
		shutil.rmtree(grader_folder)
		db.session.commit()
		return { "success": 1, "message": "Success!" }
	raise WebException("Problem does not exist!")

@blueprint.route("/update", methods=["POST"])
@admins_only
@api_wrapper
def problem_update():
	pid = request.form["pid"]
	title = request.form["title"]
	category = request.form["category"]
	description = request.form["description"]
	hint = request.form["hint"]
	value = request.form["value"]
	bonus = request.form["bonus"]
	grader_contents = request.form["grader_contents"]

	problem = Problems.query.filter_by(pid=pid).first()
	if problem:
		problem.title = title
		problem.category = category
		problem.description = description
		problem.hint = hint
		problem.value = value
		problem.bonus = bonus

		try:
			exec(grader_contents)
		except Exception, e:
			raise WebException("There is a syntax error in the grader: %s" % e)

		with open(problem.grader, "w") as grader:
			grader.write(grader_contents)
			grader.close()

		db.session.add(problem)
		db.session.commit()

		return { "success": 1, "message": "Success!" }
	raise WebException("Problem does not exist!")

@blueprint.route("/submit", methods=["POST"])
@api_wrapper
@login_required
def problem_submit():
	pid = request.form["pid"]
	flag = request.form["flag"]
	tid = session["tid"]

	problem = Problems.query.filter_by(pid=pid).first()
	team = Teams.query.filter_by(tid=tid).first()
	if problem:
		if flag == problem.flag:
			solve = Solves(pid, tid)
			team.score += problem.value
			problem.solves += 1
			db.session.add(solve)
			db.session.add(team)
			db.session.add(problem)
			db.session.commit()

			logger.log(__name__, logger.WARNING, "%s has solved %s by submitting %s" % (team.name, problem.title, flag))
			return { "success": 1, "message": "Correct!" }

		else:
			logger.log(__name__, logger.WARNING, "%s has incorrectly submitted %s to %s" % (team.name, flag, problem.title))
			raise WebException("Incorrect.")

	else:
		raise WebException("Problem does not exist!")

@blueprint.route("/data", methods=["GET"])
@api_wrapper
@admins_only
def problem_data():
	problems = Problems.query.order_by(Problems.value).all()
	problems_return = [ ]
	for problem in problems:
		problems_return.append({
			"pid": problem.pid,
			"title": problem.title,
			"category": problem.category,
			"description": problem.description,
			"hint": problem.hint,
			"value": problem.value,
			"threshold": problem.threshold,
			"weightmap": problem.weightmap,
			"grader_contents": open(problem.grader, "r").read(),
			"bonus": problem.bonus
		})
	return { "success": 1, "problems": problems_return }

def insert_problem(data, force=False):
	with app.app_context():
		if len(list(get_problem(pid=data["pid"]).all())) > 0:
			if force == True:
				_problem = Problems.query.filter_by(pid=data["pid"]).first()
				db.session.delete(_problem)
				db.session.commit()
			else:
				raise InternalException("Problem already exists.")

		insert = Problems(data["pid"], data["title"], data["category"], data["description"], data["value"])
		if "hint" in data: insert.hint = data["hint"]
		if "autogen" in data: insert.autogen = data["autogen"]
		if "bonus" in data: insert.bonus = data["bonus"]
		if "threshold" in data: insert.threshold = data["threshold"]
		if "weightmap" in data: insert.weightmap = data["weightmap"]
		db.session.add(insert)
		db.session.commit()

	return True

def get_problem(title=None, pid=None):
	match = {}
	if title != None:
		match.update({ "title": title })
	elif pid != None:
		match.update({ "pid": pid })
	with app.app_context():
		result = Problems.query.filter_by(**match)
		return result
