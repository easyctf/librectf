from flask_sqlalchemy import SQLAlchemy

import time
import traceback
import os
import base64
import onetimepass
import markdown2
import cPickle as pickle

db = SQLAlchemy()

def generate_user_link(username):
	return "<a href='/profile/%s'>%s</a>" % (username, username)

def generate_team_link(teamname):
	return "<a href='/team?teamname=%s'>%s</a>" % (teamname, teamname)

bonuses = [
	[0, 0, 0],
	[3, 2, 1],
	[5, 3, 1],
	[8, 5, 3],
	[10, 8, 6],
	[20, 12, 8],
]

class Config(db.Model):
	cfid = db.Column(db.Integer, primary_key=True)
	key = db.Column(db.String(32))
	value = db.Column(db.Text)

	def __init__(self, key, value):
		self.key = key
		self.value = value

import utils # Prevent import loops

class Users(db.Model):
	uid = db.Column(db.Integer, unique=True, primary_key=True)
	tid = db.Column(db.Integer, default=-1)
	name = db.Column(db.String(64))
	username = db.Column(db.String(64), unique=True)
	username_lower = db.Column(db.String(64), unique=True)
	email = db.Column(db.String(64), unique=True)
	password = db.Column(db.String(128))
	admin = db.Column(db.Boolean)
	utype = db.Column(db.Integer)
	registertime = db.Column(db.Integer)
	reset_token = db.Column(db.String(64))
	otp_secret = db.Column(db.String(16))
	otp_confirmed = db.Column(db.Boolean)
	email_verified = db.Column(db.Boolean)
	email_token = db.Column(db.String(64))

	def __init__(self, name, username, email, password, utype=1, admin=False):
		self.name = name
		self.username = username
		self.username_lower = username.lower()
		self.email = email.lower()
		self.password = utils.hash_password(password)
		self.utype = utype
		self.admin = admin
		self.registertime = int(time.time())
		self.otp_confirmed = False
		self.otp_secret = None
		self.email_verified = False

	def get_totp_uri(self):
		if self.otp_secret is None:
			secret = base64.b32encode(os.urandom(10)).decode("utf-8").lower()
			# self.otp_secret = base64.b32encode(os.urandom(10)).decode("utf-8").lower()
			self.otp_secret = secret
			Users.query.filter_by(uid=self.uid).update({ "otp_secret": secret })
			db.session.commit()
		service_name = utils.get_ctf_name()
		return "otpauth://totp/%s:%s?secret=%s&issuer=%s" % (service_name, self.username, self.otp_secret, service_name)

	def verify_totp(self, token):
		print self
		print "SECRET", self.otp_secret
		return onetimepass.valid_totp(token, self.otp_secret)

	def get_invitations(self):
		invitations = db.session.query(TeamInvitations).filter_by(rtype=0, toid=self.uid).all()
		result = [ ]
		for inv in invitations:
			team = db.session.query(Teams).filter_by(tid=inv.frid).first()
			result.append({
				"team": team.teamname,
				"tid": team.tid,
				"observer": team.is_observer()
			})
		return result

	def get_activity(self):
		activity = db.session.query(UserActivity).filter_by(uid=self.uid).order_by(UserActivity.timestamp.desc()).all()
		result = [ ]
		for a in activity:
			result.append({
				"timestamp": utils.isoformat(a.timestamp),
				"message": str(a)
			})
		return result

	def tfa_enabled(self):
		return self.otp_confirmed == True

	def get_stats(self):
		result = { "problems": [] }
		n_solved = [0, 0]
		for solve in list(Solves.query.filter_by(uid=self.uid).all()):
			if solve.correct == True:
				n_solved[0] += 1
				problem = Problems.query.filter_by(pid=solve.pid).first()
				result["problems"].append({ "title": problem.title, "value": solve.get_value(), "category": problem.category, "date": utils.isoformat(float(solve.date)) })
			n_solved[1] += 1
		result["correct_submissions"] = n_solved[0]
		result["total_submissions"] = n_solved[1]
		return result

class UserActivity(db.Model):
	"""
	Types of user activity:
	- 0: User joins.
	- 1: User created a team.
	- 2: User left a team.
	- 3: User solved a problem.
	"""
	uaid = db.Column(db.Integer, unique=True, primary_key=True)
	uid = db.Column(db.Integer, db.ForeignKey("users.uid"))
	type = db.Column(db.Integer)
	tid = db.Column(db.Integer, db.ForeignKey("teams.tid"))
	timestamp = db.Column(db.Integer)
	pid = db.Column(db.String(128), db.ForeignKey("problems.pid"))

	def __init__(self, uid, atype, tid=None, pid=None):
		self.uid = uid
		self.type = atype
		if tid is not None:
			self.tid = tid
		if pid is not None:
			self.pid = pid
		self.timestamp = int(time.time())

	def __str__(self):
		u = db.session.query(Users).filter_by(uid=self.uid).first()
		t = db.session.query(Teams).filter_by(tid=self.tid).first()
		p = db.session.query(Problems).filter_by(pid=self.pid).first()
		if self.type == 0:
			return "%s created an account!" % generate_user_link(u.username)
		elif self.type == 1:
			return "%s created the team %s" % (generate_user_link(u.username), generate_team_link(t.teamname))
		elif self.type == 2:
			return "%s has left team %s" % (generate_user_link(u.username), generate_team_link(t.teamname))
		elif self.type == 3:
			return "%s has solved %s" % (generate_user_link(u.username), p.title)

class Teams(db.Model):
	tid = db.Column(db.Integer, primary_key=True)
	teamname = db.Column(db.String(64), unique=True)
	teamname_lower = db.Column(db.String(64), unique=True)
	school = db.Column(db.Text)
	owner = db.Column(db.Integer)
	observer = db.Column(db.Boolean)
	finalized = db.Column(db.Boolean)

	def __init__(self, teamname, school, owner, observer):
		self.teamname = teamname
		self.teamname_lower = teamname.lower()
		self.school = school
		self.owner = owner
		self.observer = observer
		self.finalized = False

	def get_members(self):
		members = [ ]
		for member in Users.query.filter_by(tid=self.tid).all():
			members.append({
				"username": member.username,
				"name": member.name,
				"captain": member.uid == self.owner,
				"type": member.utype,
				"admin": member.admin == True,
				"observer": member.utype == 3
			})
		return members

	def points(self):
		points = 0

		# TODO: Make this better
		solves = Solves.query.filter_by(tid=self.tid, correct=1).all()
		for solve in solves:
			problem = Problems.query.filter_by(pid=solve.pid).first()
			multiplier = 1
			if solve.bonus != -1:
				multiplier += bonuses[problem.bonus][solve.bonus-1]/100.0
			points += round(problem.value*multiplier)

		return points

	def place(self, ranked=True):
		score = db.func.sum(Problems.value).label("score")
		quickest = db.func.max(Solves.date).label("quickest")
		teams = db.session.query(Solves.tid).join(Teams).join(Problems).filter().group_by(Solves.tid).order_by(score.desc(), quickest).all()
		try:
			i = teams.index((self.tid,)) + 1
			k = i % 10
			return (i, "%d%s" % (i, "tsnrhtdd"[(i / 10 % 10 != 1) * (k < 4) * k::4]))
		except ValueError:
			return (-1, "--")

	def get_invitation_requests(self, frid=None):
		if frid is not None:
			req = db.session.query(TeamInvitations).filter_by(rtype=1, frid=frid, toid=self.tid).first()
			if req is None:
				return None
			else:
				user = db.session.query(Users).filter_by(uid=req.frid).first()
				return { "username": user.username, "name": user.name, "uid": user.uid }
		result = [ ]
		requests = db.session.query(TeamInvitations).filter_by(rtype=1, toid=self.tid).all()
		for req in requests:
			user = db.session.query(Users).filter_by(uid=req.frid).first()
			result.append({
				"username": user.username,
				"name": user.name,
				"uid": user.uid
			})
		return result

	def get_pending_invitations(self, toid=None):
		if toid is not None:
			invitation = db.session.query(TeamInvitations).filter_by(rtype=0, frid=self.tid, toid=toid).first()
			if invitation is None:
				return None
			else:
				user = db.session.query(Users).filter_by(uid=invitation.toid).first()
				return { "username": user.username, "name": user.name, "uid": user.uid }
		result = [ ]
		invitations = db.session.query(TeamInvitations).filter_by(rtype=0, frid=self.tid).all()
		for invitation in invitations:
			user = db.session.query(Users).filter_by(uid=invitation.toid).first()
			result.append({
				"username": user.username,
				"name": user.name,
				"uid": user.uid
			})
		return result

	def is_observer(self):
		members = self.get_members()
		for member in members:
			if member["observer"] or member["admin"]:
				return True
		return False

	def get_last_solved(self):
		latest = 0
		return Solves.query.filter_by(tid=self.tid, correct=1).order_by(Solves.date.desc()).first().date

	def update_info(self, to_update):
		Teams.query.filter_by(tid=self.tid).update(to_update)
		db.session.commit()

	def finalize(self):
		Teams.query.filter_by(tid=self.tid).update({ "finalized": True })
		db.session.commit()

	def get_solves(self):
		solves = Solves.query.filter_by(tid=self.tid, correct=1).all()
		result = []
		for solve in solves:
			prob = Problems.query.filter_by(pid=solve.pid).first()
			result.append({
				"date": utils.isoformat(float(solve.date)),
				"problem": prob.title,
				"value": solve.get_value(),
				"solved_by": Users.query.filter_by(uid=solve.uid).first().username
			})
		return result

	def get_info(self):
		place_number, place = self.place()
		result = {
			"tid": self.tid,
			"teamname": self.teamname,
			"school": self.school,
			"place": place,
			"place_number": place_number,
			"points": self.points(),
			"members": self.get_members(),
			"captain": self.owner,
			"observer": self.is_observer(),
			"finalized": self.finalized,
			"solves": self.get_solves()
		}
		return result

class Problems(db.Model):
	pid = db.Column(db.String(128), primary_key=True, autoincrement=False)
	title = db.Column(db.String(128))
	category = db.Column(db.String(128))
	description = db.Column(db.Text)
	value = db.Column(db.Integer)
	hint = db.Column(db.Text)
	grader = db.Column(db.Text)
	autogen = db.Column(db.Boolean)
	bonus = db.Column(db.Integer)
	threshold = db.Column(db.Integer)
	weightmap = db.Column(db.PickleType)

	def __init__(self, pid, title, category, description, value, grader, hint="", autogen=False, bonus=0, threshold=0, weightmap={}):
		self.pid = pid
		self.title = title
		self.category = category
		self.description = description
		self.value = value
		self.hint = hint
		self.grader = grader
		self.autogen = autogen
		self.bonus = bonus
		self.threshold = threshold
		self.weightmap = weightmap

class Files(db.Model):
	fid = db.Column(db.Integer, primary_key=True)
	pid = db.Column(db.Integer)
	location = db.Column(db.Text)

	def __init__(self, pid, location):
		self.pid = pid
		self.location = location

class Solves(db.Model):
	sid = db.Column(db.Integer, primary_key=True)
	pid = db.Column(db.String(128), db.ForeignKey("problems.pid"))
	tid = db.Column(db.Integer, db.ForeignKey("teams.tid"))
	uid = db.Column(db.Integer)
	date = db.Column(db.String(64), default=utils.get_time_since_epoch())
	correct = db.Column(db.Boolean)
	flag = db.Column(db.Text)
	bonus = db.Column(db.Integer)

	def __init__(self, pid, uid, tid, flag, correct):
		self.pid = pid
		self.uid = uid
		self.tid = tid
		self.flag = flag
		self.correct = correct

	def get_value(self):
		problem = Problems.query.filter_by(pid=self.pid).first()
		multiplier = 1
		if self.bonus != -1:
			multiplier += bonuses[problem.bonus][self.bonus-1]/100.0
		return problem.value * multiplier

class LoginTokens(db.Model):
	TOKEN_LIFETIME = 5259492

	sid = db.Column(db.String(64), unique=True, primary_key=True)
	uid = db.Column(db.Integer, db.ForeignKey("users.uid"))
	username = db.Column(db.String(32), db.ForeignKey("users.username"))
	active = db.Column(db.Boolean)
	issued = db.Column(db.Integer)
	expiry = db.Column(db.Integer)
	ua = db.Column(db.String(128))
	ip = db.Column(db.String(16))
	location = db.Column(db.String(128))

	def __init__(self, uid, username, expiry=int(time.time()) + TOKEN_LIFETIME, active=True, ua=None, ip=None, location=None):
		self.sid = utils.generate_string()
		self.uid = uid
		self.username = username
		self.issued = int(time.time())
		self.expiry = expiry
		self.active = active
		self.ua = ua
		self.ip = ip
		self.location = location

class TeamInvitations(db.Model):
	rid = db.Column(db.Integer, primary_key=True)
	rtype = db.Column(db.Integer)
	frid = db.Column(db.Integer)
	toid = db.Column(db.Integer)
	date = db.Column(db.Integer, default=utils.get_time_since_epoch())

	def __init__(self, rtype, frid, toid):
		self.rtype = rtype
		self.frid = frid
		self.toid = toid

class Tickets(db.Model):
	htid = db.Column(db.Integer, primary_key=True)
	date = db.Column(db.Integer, default=utils.get_time_since_epoch())
	opened = db.Column(db.Boolean, default=True)
	author = db.Column(db.Integer, db.ForeignKey("users.uid"))
	title = db.Column(db.Text)
	body = db.Column(db.Text)

	def __init__(self, title, body, author):
		self.title = title
		self.body = body
		self.author = author

	def get_replies(self):
		replies = []
		for reply in TicketReplies.query.filter_by(htid=self.htid).all():
			uid = reply.author
			user = Users.query.filter_by(uid=uid).first()
			if user is not None:
				username = user.username
			else:
				username = ""

			replies.append({
				"trid": reply.trid,
				"body": markdown2.markdown(reply.body),
				"date": utils.isoformat(reply.date),
				"uid": uid,
				"username": username
			})
		return replies

class TicketReplies(db.Model):
	trid = db.Column(db.Integer, primary_key=True)
	htid = db.Column(db.Integer, db.ForeignKey("tickets.htid"))
	date = db.Column(db.Integer, default=utils.get_time_since_epoch())
	author = db.Column(db.Integer, db.ForeignKey("users.uid"))
	body = db.Column(db.Text)

	def __init__(self, htid, body, author):
		self.htid = htid
		self.body = body
		self.author = author

class ProgrammingSubmissions(db.Model):
	psid = db.Column(db.Integer, primary_key=True)
	pid = db.Column(db.String(128), db.ForeignKey("problems.pid"))
	tid = db.Column(db.Integer, db.ForeignKey("teams.tid"))
	date = db.Column(db.Integer, default=utils.get_time_since_epoch())
	message = db.Column(db.Text)
	log = db.Column(db.Text)
	submission_path = db.Column(db.Text)
	number = db.Column(db.Integer)
	duration = db.Column(db.Float)

	def __init__(self, pid, tid, submission_path, message, log, number, duration):
		self.pid = pid
		self.tid = tid
		self.submission_path = submission_path
		self.message = message
		self.log = log
		self.number = number
		self.duration = duration

class Pages(db.Model):
	pgid = db.Column(db.Integer, primary_key=True)
	pvid = db.Column(db.Integer)
	title = db.Column(db.String(256))
	content = db.Column(db.Text)

	def __init__(self, title, content, pvid = -1):
		self.pvid = pvid
		self.title = title
		self.content = content

	def get_all_pages():
		pages = list(Pages.query.filter_by().all())
		page_results = []
		current_page = filter(lambda x: x.pvid == -1, pages)[0]
		while len(pages) > 0:
			pages.remove(current_page)
			page_results.append(current_page)
			current_page = filter(lambda x: x.pvid == current_page.pgid, pages)[0]
		return page_results

	def get_html(self):
		return markdown2.markdown(self.content)