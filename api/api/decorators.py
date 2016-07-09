from functools import wraps
from flask import abort, request, session, make_response, current_app

import json
import traceback

import utils

class WebException(Exception): pass
class InternalException(Exception): pass
response_header = { "Content-Type": "application/json; charset=utf-8" }

def webhook_wrapper(f):
	@wraps(f)
	def wrapper(*args, **kwds):
		web_result = {}
		response = 200
		try:
			web_result = f(*args, **kwds)
		except WebException as error:
			response = 500
			web_result = { "success": 0, "message": str(error) }
		except Exception as error:
			response = 500
			traceback.print_exc()
			web_result = { "success": 0, "message": "Something went wrong! Please notify us about this immediately.", "error": [ str(error), traceback.format_exc() ] }
		result = (json.dumps(web_result), response, response_header)
		response = make_response(result)
		return response
	return wrapper

def api_wrapper(f):
	@wraps(f)
	def wrapper(*args, **kwds):
		if not("TESTING" in current_app.config and current_app.config["TESTING"] == True):
			if request.method == "POST":
				try:
					token = str(session.pop("csrf_token"))
					provided_token = str(request.form.get("csrf_token"))
					if not token or token != provided_token:
						raise Exception
				except Exception, e:
					response = make_response(json.dumps({ "success": 0, "message": "Token has been tampered with." }), 403, response_header)
					token = utils.generate_string()
					response.set_cookie("csrf_token", token)
					session["csrf_token"] = token
					return response

		web_result = {}
		response = 200
		try:
			web_result = f(*args, **kwds)
		except WebException as error:
			response = 200
			web_result = { "success": 0, "message": str(error) }
		except Exception as error:
			response = 200
			traceback.print_exc()
			web_result = { "success": 0, "message": "Something went wrong! Please notify us about this immediately.", "error": [ str(error), traceback.format_exc() ] }
		result = (json.dumps(web_result), response, response_header)
		response = make_response(result)

		# Setting CSRF token
		if "csrf_token" not in session:
			token = utils.generate_string()
			response.set_cookie("csrf_token", token)
			session["csrf_token"] = token

		return response
	return wrapper

def login_required(f):
	@wraps(f)
	def wrapper(*args, **kwds):
		if not user.is_logged_in():
			return { "success": 0, "message": "Not logged in." }
		return f(*args, **kwds)
	return wrapper

def team_required(f):
	@wraps(f)
	def wrapper(*args, **kwds):
		if "tid" not in session or session["tid"] < 0:
			return { "success": 0, "message": "You need a team." }
		return f(*args, **kwds)
	return wrapper

def team_finalize_required(f):
	@wraps(f)
	def wrapper(*args, **kwds):
		if team.get_team(tid=session["tid"]).first().finalized != True:
			return { "success": 0, "message": "Your team must be finalized to view this content!" }
		return f(*args, **kwds)
	return wrapper

import user # Must go below api_wrapper to prevent import loops
import team

def admins_only(f):
	@wraps(f)
	def wrapper(*args, **kwds):
		if not user.is_admin():
			return { "success": 0, "message": "Not authorized." }
		return f(*args, **kwds)
	return wrapper

def email_verification_required():
	@wraps(f)
	def wrapper(*args, **kwds):
		if not user.is_email_verified():
			return { "success": 0, "message": "Email not verified." }
		return f(*args, **kwds)
	return wrapper
