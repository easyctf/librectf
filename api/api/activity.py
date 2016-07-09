from flask import Blueprint, request
from decorators import api_wrapper, WebException
from user import get_user

import utils

blueprint = Blueprint("activity", __name__)

@blueprint.route("/user", methods=["GET"])
@api_wrapper
def activity_user():
	params = utils.flat_multi(request.args)
	if "user" not in params:
		raise WebException("Please specify a user.")
	_user = get_user(username_lower=params.get("user").lower()).first()
	if _user is None:
		raise WebException("User not found.")
	return _user.get_activity()
