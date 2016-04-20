from flask import Blueprint, jsonify
from flask import current_app as app
from decorators import admins_only, api_wrapper
from models import db, Problems, Files, Config
from schemas import verify_to_schema, check

import user
import team
import utils

blueprint = Blueprint("admin", __name__)

@blueprint.route("/setup", methods=["POST"])
@api_wrapper
def admin_setup():
	verification = Config("setup_verification", utils.generate_string().lower())
	with app.app_context():
		for item in Config.query.filter_by(key="setup_verification").all():
			db.session.delete(item)
		db.session.add(verification)
		db.session.commit()
		return { "success": 1, "verification": verification.value }

@blueprint.route("/stats/overview")
@api_wrapper
@admins_only
def admin_stats_overview():
	overview = { }
	overview["num_users"] = user.num_users(), user.num_users(observer=True)
	overview["num_teams"] = team.num_teams(), team.num_teams(observer=True)
	return { "success": 1, "overview": overview }