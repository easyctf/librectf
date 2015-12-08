from flask import abort, Blueprint, redirect, render_template, request, session
from jinja2.exceptions import TemplateNotFound
from passlib.hash import bcrypt_sha256
from OpenCTF.common import db_conn, token
from OpenCTF.utils import is_setup, sha512

import os
import re
import time

blueprint = Blueprint("views", __name__)

@blueprint.before_request
def csrf():
	if request.method == "POST":
		if session["nonce"] != request.form.get("nonce"):
			abort(403)

@blueprint.before_request
def redirect_setup():
	if not is_setup() and request.path.find("/setup") != 0:
		return redirect("/setup")
		
@blueprint.route("/setup", methods=["GET", "POST"])
def setup_page():
	if not is_setup():
		if not session.get("nonce"):
			session["nonce"] = sha512(os.urandom(10))
		if request.method == "POST":
			# print request.form
			db = db_conn()
			errors = [ ]
			config = [ ]
			
			verification = request.form.get("verification")
			actual_verification = db.config.find_one({ "key": "setup_verification" })["value"]
			if not(verification is not None and verification == actual_verification):
				errors.append("Verification is not correct.")
			
			if len(errors) == 0:
				# db.config.remove({ "key": "setup_verification" })
				if request.form.get("ctf_name") is None:
					errors.append("Please enter a name for your CTF.")
				else:
					ctf_name = request.form.get("ctf_name")
					if not(len(ctf_name) >= 4 and len(ctf_name) <= 20):
						errors.append("CTF Name must be between 4 and 20 characters long.")
					config.append({ "key": "ctf_name", "value": ctf_name })
				if request.form.get("ctf_start") is None or request.form.get("ctf_end") is None:
					errors.append("Please fill out the start and end times.")
				else:
					try:
						ctf_start_time = time.strptime(request.form.get("ctf_start"), "%m/%d/%Y %I:%M %p")
						ctf_start = time.mktime(ctf_start_time)
						ctf_end_time = time.strptime(request.form.get("ctf_end"), "%m/%d/%Y %I:%M %p")
						ctf_end = time.mktime(ctf_end_time)
						
						config.append({ "key": "ctf_start", "value": ctf_start })
						config.append({ "key": "ctf_end", "value": ctf_end })
					except:
						errors.append("Please use the correct format.")
				if request.form.get("username") is None or request.form.get("password") is None or request.form.get("email") is None:
					errors.append("Please fill out the admin details.")
				else:
					email = request.form.get("email").lower()
					if not re.match("[^@]+@[^@]+\.[^@]+", email):
						errors.append("Email is not valid.")
					if db.users.count({ "email": email }) > 0:
						errors.append("That email is taken.")
					config.append({ "key": "admin_email", "value": email })
					username = request.form.get("username")
					if not(len(username) >= 4 and len(username) <= 20):
						errors.append("Username must be between 4 and 20 characters long.")
					if db.users.count({ "username_lower": username.lower() }) > 0:
						errors.append("That username is taken.")
					config.append({ "key": "admin_username", "value": username.lower() })
					password = request.form.get("password")
					if not(len(password) >= 6 and len(password) <= 60):
						errors.append("Password must be between 6 and 60 characters long.")
					password = bcrypt_sha256.encrypt(password)
					
					if len(errors) != 0:
						admin = {
							"uid": token(),
							"email": email,
							"username": username,
							"username_lower": username.lower(),
							"password": password,
						}
						db.users.insert(admin)
			
			if len(errors) != 0:
				return render_template("setup.html", nonce=session.get("nonce"), errors=errors, data=request.form)
			else:
				for obj in config:
					db.config.update({ "key": obj["key"] }, obj, upsert=True)
				db.config.update({ "key": "setup_complete" },
					{ "key": "setup_complete", "value": True }, upsert=True)
				return redirect("/")
		else:
			db = db_conn()
			if db.config.count({ "key": "setup_verification" }) == 0:
				db.config.insert({ "key": "setup_verification", "value": token() })
			return render_template("setup.html", nonce=session.get("nonce"))
	else:
		return redirect("/")

@blueprint.route("/", defaults={ "template": "index" })
@blueprint.route("/<template>")
def static_html(template):
	try:
		return render_template("%s.html" % template)
	except TemplateNotFound:
		abort(404)