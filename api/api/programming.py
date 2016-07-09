from flask import current_app as app, Blueprint, request, session

from decorators import api_wrapper, login_required, team_required, WebException
from models import db, Problems, ProgrammingSubmissions, Solves, UserActivity

import imp
import os
import shutil
import subprocess
import time

import cache
import problem
import team
import user
import utils

blueprint = Blueprint("programming", __name__)

extensions = {
	"python2": "py",
	"python3": "py",
	"java": "java"
}

@blueprint.route("/submissions/delete", methods=["POST"])
@api_wrapper
@login_required
@team_required
def delete_submission():
	params = utils.flat_multi(request.form)
	psid = params.get("psid")
	tid = session.get("tid")
	result = ProgrammingSubmissions.query.filter_by(psid=psid, tid=tid)

	if result.first() is None:
		raise WebException("Submission does not exist.")

	with app.app_context():
		result.delete()
		db.session.commit()

	return { "success": 1, "message": "Success!" }

@blueprint.route("/submissions", methods=["GET"])
@api_wrapper
@login_required
@team_required
def get_submissions():
	submissions_return = []
	tid = session.get("tid")
	submissions = ProgrammingSubmissions.query.filter_by(tid=tid).order_by(ProgrammingSubmissions.psid.desc()).all()
	if submissions is not None:
		for submission in submissions:
			_problem = problem.get_problem(pid=submission.pid).first()
			submissions_return.append({
				"psid": submission.psid,
				"title": _problem.title if _problem else "",
				"message": submission.message,
				"log": submission.log,
				"date": utils.isoformat(submission.date),
				"number": submission.number,
				"duration": submission.duration
			})
	return { "success": 1, "submissions": submissions_return }

@blueprint.route("/problems", methods=["GET"])
@api_wrapper
@login_required
def get_problems():
	if session.get("admin"):
		pass
	elif session.get("tid") <= 0:
		raise WebException("You need a team.")
	elif team.get_team(tid=session.get("tid")).first().finalized != True:
		raise WebException("Your team is not finalized.")

	data = []
	problems = Problems.query.filter_by(category="Programming").all()
	if problems is not None:
		for _problem in problems:
			data.append({
				"title": _problem.title,
				"pid": _problem.pid,
				"value": _problem.value
			})

	return { "success": 1, "problems": data }

@blueprint.route("/submit", methods=["POST"])
@api_wrapper
@login_required
@team_required
def submit_program():
	params = utils.flat_multi(request.form)

	pid = params.get("pid")
	tid = session.get("tid")
	_user = user.get_user().first()

	language = params.get("language")
	submission_contents = params.get("submission")

	_problem = problem.get_problem(pid=pid).first()
	if _problem is None:
		raise WebException("Problem does not exist.")

	if _problem.category != "Programming":
		raise WebException("Can't judge this problem.")

	if language not in extensions:
		raise WebException("Language not supported.")

	solved = Solves.query.filter_by(pid=pid, tid=tid, correct=1).first()
	if solved:
		raise WebException("You already solved this problem.")

	judge_folder = os.path.join(app.config["GRADER_FOLDER"], pid)
	if not os.path.exists(judge_folder):
		os.makedirs(judge_folder)

	submission_folder = os.path.join(judge_folder, utils.generate_string())
	while os.path.exists(submission_folder):
		submission_folder = os.path.join(judge_folder, utils.generate_string())

	os.makedirs(submission_folder)

	submission_path = os.path.join(submission_folder, "program.%s" % extensions[language])

	open(submission_path, "w").write(submission_contents)
	message, log, duration = judge(submission_path, language, pid)

	number = ProgrammingSubmissions.query.filter_by(tid=tid).with_entities(ProgrammingSubmissions.number).order_by(ProgrammingSubmissions.number.desc()).first()

	if number is None:
		number = 1
	else:
		number = number[0] + 1

	submission = ProgrammingSubmissions(pid, tid, submission_path, message, log, number, duration)

	correct = message == "Correct!"

	with app.app_context():
		solve = Solves(pid, _user.uid, tid, submission_path, correct)
		db.session.add(solve)
		db.session.add(submission)
		db.session.commit()

		if correct:
			# Wait until after the solve has been added to the database before adding bonus
			solves = problem.get_solves(pid=pid)
			solve.bonus = [-1, solves][solves < 3]
			db.session.add(solve)
			cache.invalidate_memoization(problem.get_solves, pid)

			if _user:
				activity = UserActivity(_user.uid, 3, tid=tid, pid=pid)
				db.session.add(activity)

			db.session.commit()
		new = {
			"psid": submission.psid,
			"title": _problem.title,
			"message": submission.message,
			"log": submission.log,
			"date": utils.isoformat(submission.date),
			"number": submission.number
		}

	shutil.rmtree(submission_folder)

	return { "success": message == "Correct!", "message": message , "new_submission": new }

def judge(submission_path, language, pid):

	if not os.path.exists(submission_path):
		raise WebException("Program is missing.")

	_problem = problem.get_problem(pid=pid).first()
	if _problem is None:
		raise WebException("Problem does not exist.")

	submission_root = os.path.dirname(submission_path)
	os.chdir(submission_root)
	log = ""
	message = ""

	log += "Compiling...\n"
	start_time = time.time()
	try:
		if language == "python2":
			subprocess.check_output("python -m py_compile %s" % submission_path, shell=True)
		elif language == "python3":
			subprocess.check_output("python3 -m py_compile %s" % submission_path, shell=True)
		elif language == "java":
			subprocess.check_output("javac %s" % submission_path, shell=True)
		else:
			message = "Not implemented."
			return message, log, time.time() - start_time
	except subprocess.CalledProcessError as e:
		# TODO: Extract useful error messages from exceptions and add timeout
		#log += "There was a problem with compiling.\n%s\n" % str(e)
		message = "There was a problem with compiling."

		return message, log, time.time() - start_time

	log += "Compiled.\n"

	try:
		judge = imp.load_source("judge", _problem.grader)
	except Exception, e:
		message = "An error occured. Please notify an admin immediately."
		log += "Could not load judge.\n"
		return message, log, time.time() - start_time

	for i in range(1, judge.TEST_COUNT + 1):
		log += "Running test #%s\n" % i

		try:
			_input, correct = judge.generate()
		except Exception, e:
			message = "An error occured. Please notify an admin immediately."
			log += "Could not generate input for test #%s.\n" % i
			return message, log, time.time() - start_time

		try:
			command = ""
			if language == "python2":
				command = "python %s <<< \"%s\"" % (submission_path, _input)
			elif language == "python3":
				command = "python3 %s <<< \"%s\"" % (submission_path, _input)
			elif language == "java":
				command = "java program <<< \"%s\"" % _input
			output = subprocess.check_output(command, shell=True, executable="/bin/bash").strip()
		except subprocess.CalledProcessError as e:
			#log += "Program threw an exception:\n%s\n" % str(e)
			message = "Program crashed."
			return message, log, time.time() - start_time

		if correct != output:
			message = "Incorrect."
			log += "Test #%s failed.\n\n" % i
			log += "Input:\n%s\n\n" % _input
			log += "Output:\n%s\n\n" % output
			log += "Expected:\n%s\n\n" % correct
			return message, log, time.time() - start_time
		else:
			log += "Test #%s passed!\n" % i

	message = "Correct!"
	log += "All tests passed."

	return message, log, time.time() - start_time

def validate_judge(judge_contents):
	tmp_judge = "/tmp/judge.py"

	open(tmp_judge, "w").write(judge_contents)

	try:
		judge = imp.load_source("judge", tmp_judge)
	except Exception, e:
		raise WebException("There is a syntax error in the judge: %s" % e)

	try:
		assert hasattr(judge, "TEST_COUNT"), "Judge missing TEST_COUNT."

		assert type(judge.TEST_COUNT) == int, "TEST_COUNT must be an integer."

		_input, correct = judge.generate()

		assert _input is not None, "Judge did not generate valid input."
		assert correct is not None, "Judge did not generate a valid response."
	except AssertionError, e:
		raise WebException(e)
	except Exception, e:
		raise WebException(e)
