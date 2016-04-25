from flask import Blueprint, request, session
from flask import current_app as app

from decorators import api_wrapper

from models import db, Problems, Solves, Teams
import team

blueprint = Blueprint("stats", __name__)

@blueprint.route("/scoreboard")
@api_wrapper
def all_teams_stats():
	score = db.func.sum(Problems.value).label("score")
	quickest = db.func.max(Solves.date).label("quickest")
	teams = db.session.query(Solves.tid, Teams).join(Teams).join(Problems).filter().group_by(Solves.tid).order_by(score.desc(), quickest).all()
	result = [ ]
	count = 0
	for tid, _team in teams:
		count += 1
		result.append({
			"rank": count,
			"teamname": _team.teamname,
			"tid": tid,
			"school": _team.school,
			"points": _team.points()
		})
	return { "success": 1, "scoreboard": result }
