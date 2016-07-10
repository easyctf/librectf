from flask import Blueprint, request, session
from flask import current_app as app

from decorators import WebException, api_wrapper

from models import db, Problems, Solves, Teams
import team

blueprint = Blueprint("stats", __name__)

@blueprint.route("/scoreboard")
@api_wrapper
def all_teams_stats():
	teams = get_leaderboard()
	result = [ ]
	count = 0
	for place, _team in teams:
		result.append({
			"rank": place,
			"teamname": _team.teamname,
			"tid": _team.tid,
			"school": _team.school,
			"points": _team.points(),
			"observer": _team.is_observer(),
			"latest": _team.get_last_solved()
		})
	return { "success": 1, "scoreboard": result }

def get_leaderboard_tids():
	teams = get_leaderboard()
	return [(place, team.tid) for place, team in teams]

def get_leaderboard():
	db.session.expire_all()
	teams = list(Teams.query.all())
	teams.sort(key=lambda x: (x.points(), -x.get_last_solved()), reverse=True)
	result = []
	count = 0
	prevPoints = 0
	for team in teams:
		points = team.points()
		if count > 0 and points == prevPoints and points == 0:
			count -= 1
		count += 1
		result.append((count, team))
		prevPoints = points
	return result