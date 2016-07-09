from flask import Blueprint, request, session
from flask import current_app as app

from decorators import api_wrapper

from models import db, Problems, Solves, Teams
import team

blueprint = Blueprint("stats", __name__)

@blueprint.route("/scoreboard")
@api_wrapper
def all_teams_stats():
	db.session.expire_all()
	score = db.func.sum(Problems.value).label("score")
	quickest = db.func.max(Solves.date).label("quickest")
	teams = list(Teams.query.filter_by().all())
	result = [ ]
	count = 0
	for _team in teams:
		if _team.finalized != True: continue
		count += 1
		result.append({
			"rank": count,
			"teamname": _team.teamname,
			"tid": _team.tid,
			"school": _team.school,
			"points": _team.points(),
			"observer": _team.is_observer(),
		})
	return { "success": 1, "scoreboard": result }
