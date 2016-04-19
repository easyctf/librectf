from flask import Blueprint, jsonify
from decorators import admins_only, api_wrapper
from models import db, Problems, Files
from schemas import verify_to_schema, check

import user
import team

blueprint = Blueprint("admin", __name__)

@blueprint.route("/stats/overview")
@api_wrapper
@admins_only
def admin_stats_overview():
	overview = { }
	overview["num_users"] = user.num_users(), user.num_users(observer=True)
	overview["num_teams"] = team.num_teams(), team.num_teams(observer=True)
	return { "success": 1, "overview": overview }