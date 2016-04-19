from flask import Blueprint, make_response, session, request, redirect, url_for, send_file, abort
from werkzeug import secure_filename
from flask import current_app as app
from voluptuous import Schema, Length, Required
from StringIO import StringIO

from models import db, LoginTokens, Users, UserActivity
from decorators import api_wrapper, WebException, login_required
from schemas import verify_to_schema, check

import datetime
import logger, logging
import os
import cache
import re
import requests
import team
import utils
import pyqrcode
import string

from PIL import Image

###############
# USER ROUTES #
###############

blueprint = Blueprint("user", __name__)

@blueprint.route("/forgot", methods=["POST"])
@blueprint.route("/forgot/<token>", methods=["GET", "POST"])
@api_wrapper
def user_forgot_password(token=None):
	params = utils.flat_multi(request.form)
	if token is not None:
		user = get_user(reset_token=token).first()
		if user is None:
			raise WebException("Invalid reset token.")

		# We are viewing the actual reset form
		if request.method == "GET":
			return { "success": 1, "message": ""}

		# Submission of actual reset form
		if request.method == "POST":
			password = params.get("password")
			confirm_password = params.get("confirm_password")
			if password != confirm_password:
				raise WebException("Passwords do not match.")
			else:
				user.password = utils.hash_password(password)
				user.reset_token = None
				current_session = db.session.object_session(user)
				current_session.add(user)
				current_session.commit()
				return { "success": 1, "message": "Success!" }
	else:
		email = params.get("email").lower()

		user = get_user(email=email).first()
		if user is None:
			raise WebException("User with that email does not exist.")

		token = utils.generate_string(length=64)
		user.reset_token = token
		current_session = db.session.object_session(user)
		current_session.add(user)
		current_session.commit()

		reset_link = "%s/forgot/%s" % ("127.0.0.1:8000", token)
		subject = "OpenCTF password reset"
		body = """%s,\n\nA request to reset your OpenCTF password has been made. If you did not request this password reset, you may safely ignore this email and delete it.\n\nYou may reset your password by clicking this link or pasting it to your browser.\n\n%s\n\nThis link can only be used once, and will lead you to a page where you can reset your password.\n\nGood luck!\n\n- OpenCTF Administrator""" % (user.username, reset_link)
		response = utils.send_email(email, subject, body).json()
		if "Queued" in response["message"]:
			return { "success": 1, "message": "Email sent to %s" % email }
		else:
			raise WebException(response["message"])

@blueprint.route("/register", methods=["POST"])
@api_wrapper
def user_register():
	params = utils.flat_multi(request.form)

	name = params.get("name")
	email = params.get("email")
	username = params.get("username")
	password = params.get("password")
	password_confirm = params.get("password_confirm")
	utype = int(params.get("type"))

	if password != password_confirm:
		raise WebException("Passwords do not match.")
	verify_to_schema(UserSchema, params)

	user = Users(name, username, email, password, utype)
	with app.app_context():
		db.session.add(user)
		db.session.commit()
		join_activity = UserActivity(user.uid, 0)
		db.session.add(join_activity)
		db.session.commit()

	logger.log(__name__, "%s registered with %s" % (name.encode("utf-8"), email.encode("utf-8")))
	login_user(username, password)

	return { "success": 1, "message": "Success!" }

@blueprint.route("/logout", methods=["GET"])
@api_wrapper
def user_logout():
	sid = session["sid"]
	username = session["username"]
	with app.app_context():
		expired = LoginTokens.query.filter_by(username=username).all()
		for expired_token in expired: db.session.delete(expired_token)
		db.session.commit()
	session.clear()

@blueprint.route("/login", methods=["POST"])
@api_wrapper
def user_login():
	params = utils.flat_multi(request.form)

	username = params.get("username")
	password = params.get("password")

	result = login_user(username, password)
	if result != True:
		raise WebException("Please check if your username/password are correct.")

	return { "success": 1, "message": "Success!" }

@blueprint.route("/status", methods=["GET"])
@api_wrapper
def user_status():
	logged_in = is_logged_in()
	result = {
		"success": 1,
		"logged_in": logged_in,
		"admin": is_admin(),
		"competition": is_admin(),
		"in_team": in_team(get_user()),
		"username": session["username"] if logged_in else "",
	}
	if logged_in:
		result["has_team"] = in_team(get_user().first())

	return result

@blueprint.route("/info", methods=["GET"])
@api_wrapper
def user_info():
	logged_in = is_logged_in()
	username = utils.flat_multi(request.args).get("username")
	if username is None:
		if logged_in:
			username = session["username"]
	if username is None:
		raise WebException("No user specified.")
	me = False if not("username" in session) else username.lower() == session["username"].lower()
	user = get_user(username_lower=username.lower()).first()
	if user is None:
		raise WebException("User not found.")

	show_email = me if logged_in else False
	user_in_team = in_team(user)
	userdata = {
		"user_found": True,
		"name": user.name,
		"username": user.username,
		"type": ["Student", "Instructor", "Observer"][user.utype - 1],
		"admin": user.admin,
		"registertime": datetime.datetime.fromtimestamp(user.registertime).isoformat() + "Z",
		"me": me,
		"show_email": show_email,
		"in_team": user_in_team,
		"uid": user.uid,
		"activity": user.get_activity()
	}
	if show_email:
		userdata["email"] = user.email
	if user_in_team:
		userdata["team"] = team.get_team_info(tid=user.tid)
	if me:
		userdata["tfa_enabled"] = user.tfa_enabled()
		if not(user_in_team):
			invitations = user.get_invitations()
			userdata["invitations"] = invitations
	return { "success": 1, "user": userdata }

@blueprint.route("/avatar/<uid>", methods=["GET"])
def user_avatar(uid):
	uid = int(uid)
	try:
		return send_file("pfp/%d.png" % uid, mimetype="image/png")
	except:
		user = get_user(uid=uid).first()
		if user is not None:
			utils.generate_identicon(user.email, user.uid)
			return send_file("pfp/%d.png" % uid, mimetype="image/png")
		return abort(404)

@blueprint.route("/twofactor/qr", methods=["GET"])
def user_twofactor_qr():
	if not is_logged_in():
		abort(404)
	user = get_user().first()
	if user is None:
		abort(404)

	url = pyqrcode.create(user.get_totp_uri())
	stream = StringIO()
	url.svg(stream, scale=6)
	return stream.getvalue().encode("utf-8"), 200, {
		"Content-Type": "image/svg+xml",
		"Cache-Control": "no-cache, no-store, must-revalidate",
		"Pragma": "no-cache",
		"Expires": 0,
		"Debug-Secret": user.otp_secret
	}

@blueprint.route("/twofactor/verify", methods=["POST"])
@login_required
@api_wrapper
def user_twofactor_verify():
	_user = get_user().first()
	if _user is None:
		raise WebException("User not found.")

	print "SECRET (1)", _user.otp_secret
	params = utils.flat_multi(request.form)
	if "token" not in params:
		raise WebException("Invalid token.")
	token = params["token"]

	if not(_user.verify_totp(token)):
		raise WebException("Invalid token.")
	with app.app_context():
		Users.query.filter_by(uid=_user.uid).update({ "otp_confirmed": True })
		db.session.commit()

	print "CONFIRMED"

	return { "success": 1, "message": "Confirmed!" }

@blueprint.route("/avatar/upload", methods=["POST"])
@api_wrapper
def user_avatar_upload():
	logged_in = is_logged_in()
	if not logged_in:
		raise WebException("You're not logged in.")

	_user = get_user().first()
	f = request.files["file"]
	if f is None:
		raise WebException("Please upload something.")

	fname = "/tmp/" + secure_filename(utils.generate_string())
	f.save(fname)

	try:
		pfp = "pfp/%d.png" % _user.uid
		os.remove(pfp)
		im = Image.open(fname)
		im = im.resize((256, 256), Image.ANTIALIAS)
		im.save(open(pfp, "w"), "PNG")
		return { "success": 1, "message": "Uploaded!" }
	except Exception, e:
		raise WebException(str(e))

@blueprint.route("/avatar/remove", methods=["POST"])
@api_wrapper
def user_avatar_remove():
	logged_in = is_logged_in()
	if not logged_in:
		raise WebException("You're not logged in.")
	_user = get_user().first()

	try:
		pfp = "pfp/%d.png" % _user.uid
		os.remove(pfp)
		return { "success": 1, "message": "Removed!" }
	except Exception, e:
		raise WebException(str(e))

##################
# USER FUNCTIONS #
##################

__check_first_character_is_letter = lambda username: username[0] in string.letters
__check_no_spaces = lambda username: all([c!=" " for c in username])
__check_username = lambda username: get_user(username_lower=username.lower()).first() is None
__check_email = lambda email: get_user(email=email.lower()).first() is None

UserSchema = Schema({
	Required("email"): check(
		([str, Length(min=4, max=128)], "Your email should be between 4 and 128 characters long."),
		([__check_email], "Someone already registered this email."),
		([utils.__check_email_format], "Please enter a legit email.")
	),
	Required("name"): check(
		([str, Length(min=4, max=128)], "Your name should be between 4 and 128 characters long.")
	),
	Required("username"): check(
		([str, Length(min=4, max=32)], "Your username should be between 4 and 32 characters long."),
		([utils.__check_alphanumeric], "Please only use alphanumeric characters in your username."),
		([__check_first_character_is_letter], "Your username must begin with a letter."),
		([__check_no_spaces], "Your username must not contain spaces."),
		([__check_username], "This username is taken, did you forget your password?")
	),
	Required("password"): check(
		([str, Length(min=4, max=64)], "Your password should be between 4 and 64 characters long."),
		([utils.__check_ascii], "Please only use ASCII characters in your password."),
	),
	Required("type"): check(
		([str, lambda x: x.isdigit()], "Please use the online form.")
	),
	"notify": str
}, extra=True)

def login_user(username, password):
	user = get_user(username_lower=username.lower()).first()
	if user is None: return False
	correct = utils.check_password(user.password, password)
	if not correct: return False

	useragent = request.headers.get("User-Agent")
	ip = request.remote_addr

	with app.app_context():
		expired = LoginTokens.query.filter_by(username=username).all()
		for expired_token in expired: db.session.delete(expired_token)

		token = LoginTokens(user.uid, user.username, ua=useragent, ip=ip)
		db.session.add(token)
		db.session.commit()

		session["sid"] = token.sid
		session["username"] = token.username
		session["admin"] = user.admin == True
		if user.tid is not None and user.tid >= 0:
			session["tid"] = user.tid

	return True

def is_logged_in():
	if not("sid" in session and "username" in session): return False
	sid = session["sid"]
	username = session["username"]
	token = LoginTokens.query.filter_by(sid=sid).first()
	if token is None: return False

	useragent = request.headers.get("User-Agent")
	ip = request.remote_addr

	if token.username != username: return False
	if token.ua != useragent: return False
	return True

def get_user(username=None, username_lower=None, email=None, uid=None, reset_token=None):
	match = {}
	if username != None:
		match.update({ "username": username })
	elif username_lower != None:
		match.update({ "username_lower": username_lower })
	elif uid != None:
		match.update({ "uid": uid })
	elif email != None:
		match.update({ "email": email })
	elif is_logged_in():
		match.update({ "username": session["username"] })
	elif reset_token != None:
		match.update({ "reset_token": reset_token })
	with app.app_context():
		result = Users.query.filter_by(**match)
		return result

def is_admin():
	return is_logged_in() and "admin" in session and session["admin"]

def in_team(user):
	return hasattr(user, "tid") and user.tid >= 0

@cache.memoize()
def num_users(observer=False):
	cursor = None
	if observer == True:
		cursor = Users.query.filter_by()
	else:
		cursor = Users.query.filter_by(utype=1)
	count = cursor.count()
	return count