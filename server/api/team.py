from flask import Blueprint, request, session
from flask import current_app as app
from voluptuous import Schema, Length, Required

from models import db, Teams, Users, TeamInvitations, UserActivity
from decorators import api_wrapper, login_required, WebException
from schemas import verify_to_schema, check

import user
import utils
import cache

blueprint = Blueprint("team", __name__)

###############
# TEAM ROUTES #
###############

@blueprint.route("/create", methods=["POST"])
@api_wrapper
@login_required
def team_create():
	params = utils.flat_multi(request.form)
	_user = user.get_user().first()
	if (_user.tid is not None and _user.tid >= 0) or get_team(owner=_user.uid).first() is not None:
		raise WebException("You're already in a team!")

	verify_to_schema(TeamSchema, params)
	teamname = params.get("teamname")
	school = params.get("school")

	team = Teams(teamname, school, _user.uid, _user.utype != 1)
	with app.app_context():
		db.session.add(team)
		db.session.commit()
		Users.query.filter_by(uid=_user.uid).update({ "tid": team.tid })
		team_activity = UserActivity(_user.uid, 1, tid=team.tid)
		db.session.add(team_activity)
		db.session.commit()

		session["tid"] = team.tid
	return { "success": 1, "message": "Success!" }

@blueprint.route("/delete", methods=["POST"])
@api_wrapper
@login_required
def team_delete():
	username = session["username"]
	tid = session["tid"]
	team = Teams.query.filter_by(tid=tid).first()
	usr = Users.query.filter_by(username=username).first()
	owner = team.owner
	if usr.uid == owner or usr.admin:
		with app.app_context():
			for member in Users.query.filter_by(tid=tid).all():
				member.tid = -1
				db.session.add(member)
				db.session.delete(team)
				db.session.commit()
			db.session.close()
			session.pop("tid")
		return { "success": 1, "message": "Success!" }
	else:
		raise WebException("Not authorized.")

@blueprint.route("/remove_member", methods=["POST"])
@api_wrapper
@login_required
def team_remove_member():
	username = session["username"]
	tid = session["tid"]
	team = Teams.query.filter_by(tid=tid).first()
	usr = Users.query.filter_by(username=username).first()
	owner = team.owner
	if usr.uid == owner or usr.admin:
		params = utils.flat_multi(request.form)
		user_to_remove = Users.query.filter_by(username=params.get("user"))
		user_to_remove.tid = -1
		with app.app_context():
			db.session.add(user_to_remove)
			db.session.commit()
			db.session.close()
		return { "success": 1, "message": "Success!" }
	else:
		raise WebException("Not authorized.")

@blueprint.route("/invite", methods=["POST"])
@api_wrapper
@login_required
def team_invite():
	params = utils.flat_multi(request.form)
	_user = user.get_user().first()
	if not user.in_team(_user):
		raise WebException("You must be in a team!")
	_team = get_team(tid=_user.tid).first()
	if _user.uid != _team.owner:
		raise WebException("You must be the captain of your team to invite members!")

	new_member = params.get("new_member")
	if new_member is None:
		raise WebException("Please provide a username!")
	_user2 = user.get_user(username_lower=new_member.lower()).first()
	if _user2 is None:
		raise WebException("User doesn't exist!")
	if _user2.tid > 0:
		raise WebException("This user is already a part of a team!")

	if _team.get_pending_invitations(toid=_user2.uid) is not None:
		raise WebException("You've already invited this member!")

	req = TeamInvitations(0, _team.tid, _user2.uid)
	with app.app_context():
		db.session.add(req)
		db.session.commit()
		db.session.close()

	return { "success": 1, "message": "Success!" }

@blueprint.route("/invite/rescind", methods=["POST"])
@api_wrapper
@login_required
def team_invite_rescind():
	params = utils.flat_multi(request.form)
	_user = user.get_user().first()
	if not user.in_team(_user):
		raise WebException("You must be in a team!")
	_team = get_team(tid=_user.tid).first()
	if _user.uid != _team.owner:
		raise WebException("You must be the captain of your team to rescind invitations!")

	uid = params.get("uid")
	if uid is None:
		raise WebException("Please provide a user.")
	invitation = TeamInvitations.query.filter_by(rtype=0, frid=_team.tid, toid=uid).first()
	if invitation is None:
		raise WebException("Invitation doesn't exist.")

	with app.app_context():
		db.session.delete(invitation)
		db.session.commit()
		db.session.close()

	return { "success": 1, "message": "Success!" }

@blueprint.route("/invite/request", methods=["POST"])
@api_wrapper
@login_required
def team_invite_request():
	params = utils.flat_multi(request.form)
	_user = user.get_user().first()
	if user.in_team(_user):
		raise WebException("You're already in a team!")

	tid = params.get("tid")
	_team = get_team(tid=tid).first()
	if _team is None:
		raise WebException("Team not found.")

	if _team.get_invitation_requests(frid=_user.uid) is not None:
		raise WebException("You've already requested to join this team!")

	req = TeamInvitations(1, _user.uid, _team.tid)
	with app.app_context():
		db.session.add(req)
		db.session.commit()
		db.session.close()

	return { "success": 1, "message": "Success!" }

@blueprint.route("/invite/accept", methods=["POST"])
@api_wrapper
def team_accept_invite():
	params = utils.flat_multi(request.form)
	_user = user.get_user().first()
	if user.in_team(_user):
		raise WebException("You're already in a team!")

	tid = params.get("tid")
	_team = get_team(tid=tid).first()
	if _team is None:
		raise WebException("Team not found.")

	if len(_team.get_members()) >= 5:
		raise WebException("This team is full.")

	invitation = TeamInvitations.query.filter_by(rtype=0, frid=tid, toid=_user.uid).first()
	if invitation is None:
		raise WebException("Invitation doesn't exist.")

	with app.app_context():
		_user = Users.query.filter_by(uid=_user.uid).first()
		_user.tid = tid
		db.session.delete(invitation)
		invitation2 = TeamInvitations.query.filter_by(rtype=1, frid=_user.uid, toid=tid).first()
		if invitation2 is not None:
			db.session.delete(invitation2)
		db.session.commit()
		db.session.close()

	return { "success": 1, "message": "Success!" }

@blueprint.route("/invite/request/accept", methods=["POST"])
@api_wrapper
def team_accept_invite_request():
	params = utils.flat_multi(request.form)
	_user = user.get_user().first()
	if not user.in_team(_user):
		raise WebException("You must be in a team!")
	_team = get_team(tid=_user.tid).first()
	tid = _team.tid
	if _user.uid != _team.owner:
		raise WebException("You must be the captain of your team to rescind invitations!")

	if len(_team.get_members()) >= 5:
		raise WebException("Your team is full.")

	uid = params.get("uid")
	_user2 = user.get_user(uid=uid).first()
	if user.in_team(_user2):
		raise WebException("This user is already in a team!")

	invitation = TeamInvitations.query.filter_by(rtype=1, frid=_user2.uid, toid=tid).first()
	if invitation is None:
		raise WebException("Invitation doesn't exist.")

	with app.app_context():
		_user2 = Users.query.filter_by(uid=_user2.uid).first()
		_user2.tid = tid
		db.session.delete(invitation)
		invitation2 = TeamInvitations.query.filter_by(rtype=0, frid=tid, toid=_user2.uid).first()
		if invitation2 is not None:
			db.session.delete(invitation2)
		db.session.commit()
		db.session.close()

	return { "success": 1, "message": "Success!" }

@blueprint.route("/info", methods=["GET"])
@api_wrapper
def team_info():
	logged_in = user.is_logged_in()
	in_team = False
	owner = False
	_user = None
	teamdata = { }
	search = { }
	teamname = utils.flat_multi(request.args).get("teamname")
	if teamname:
		search.update({ "teamname_lower": teamname.lower() })
	if logged_in:
		_user = user.get_user().first()
		if user.in_team(_user):
			if "teamname_lower" not in search:
				search.update({ "tid": _user.tid })
				in_team = True
	if bool(search) != False:
		team = get_team(**search).first()
		teamdata = get_team_info(**search)
		if logged_in:
			in_team = teamdata["tid"] == _user.tid
			owner = teamdata["captain"] == _user.uid
		teamdata["in_team"] = in_team
		if in_team:
			teamdata["is_owner"] = owner
			if owner:
				teamdata["pending_invitations"] = team.get_pending_invitations()
				teamdata["invitation_requests"] = team.get_invitation_requests()
		else:
			if logged_in:
				teamdata["invited"] = team.get_pending_invitations(toid=_user.uid) is not None
				teamdata["requested"] = team.get_invitation_requests(frid=_user.uid) is not None
	else:
		if logged_in:
			teamdata["invitations"] = _user.get_invitations()
	return { "success": 1, "team": teamdata }

##################
# TEAM FUNCTIONS #
##################

__check_teamname = lambda teamname: get_team(teamname_lower=teamname.lower()).first() is None

TeamSchema = Schema({
	Required("teamname"): check(
		([str, Length(min=4, max=32)], "Your teamname should be between 4 and 32 characters long."),
		([utils.__check_ascii], "Please only use ASCII characters in your teamname."),
		([__check_teamname], "This teamname is taken, did you forget your password?")
	),
	Required("school"): check(
		([str, Length(min=4, max=40)], "Your school name should be between 4 and 40 characters long."),
		([utils.__check_ascii], "Please only use ASCII characters in your school name."),
	),
}, extra=True)

def get_team_info(tid=None, teamname=None, teamname_lower=None, owner=None):
	team = get_team(tid=tid, teamname=teamname, teamname_lower=teamname_lower, owner=owner).first()
	if team is None:
		raise WebException("Team not found.")

	place_number, place = team.place()
	result = {
		"tid": team.tid,
		"teamname": team.teamname,
		"school": team.school,
		"place": place,
		"place_number": place_number,
		"points": team.points(),
		"members": team.get_members(),
		"captain": team.owner,
		"observer": team.is_observer()
	}
	return result

def get_team(tid=None, teamname=None, teamname_lower=None, owner=None):
	match = {}
	if teamname != None:
		match.update({ "teamname": teamname })
	elif teamname_lower != None:
		match.update({ "teamname_lower": teamname_lower })
	elif tid != None:
		match.update({ "tid": tid })
	elif owner != None:
		match.update({ "owner": owner })
	#elif user.is_logged_in():
	#	_user = user.get_user().first()
	#	if _user.tid is not None:
	#		match.update({ "tid": _user.tid })
	with app.app_context():
		result = Teams.query.filter_by(**match)
		return result

@cache.memoize()
def num_teams(observer=False):
	teamlist = list(get_team().all())
	if observer == False:
		teamlist = filter(lambda t: t.is_observer() == False, teamlist)
	return len(teamlist)
