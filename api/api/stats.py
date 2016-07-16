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
	for place, unranked_place, _team in teams:
		result.append({
			"rank": place,
			"rank_all": unranked_place,
			"teamname": _team.teamname,
			"tid": _team.tid,
			"school": _team.school,
			"points": _team.points(),
			"observer": _team.is_observer(),
			"latest": _team.get_last_solved()
		})
	return { "success": 1, "scoreboard": result }

def get_leaderboard_tids(ranked=True):
	teams = get_leaderboard()
	result = [(place if ranked == True else unranked_place, team.tid) if not(ranked == True and team.is_observer()) else None for place, unranked_place, team in teams]
	return filter(lambda x: x is not None, result)

def get_leaderboard():
	db.session.expire_all()
	teams = list(Teams.query.all())
	teams.sort(key=lambda x: (x.points(), -x.get_last_solved()), reverse=True)
	result = []
	count, ranked_count = 0, 0
	prevPoints = 0
	for team in teams:
		points = team.points()
		if points == prevPoints and points == 0:
			if count > 0:
				count -= 1
			if ranked_count > 0:
				ranked_count -= 1
		if team.is_observer():
			ranked_count -= 1
		ranked_count += 1
		count += 1
		result.append((ranked_count, count, team))
		prevPoints = points
	return result