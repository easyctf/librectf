from flask import Blueprint, request, session
from flask import current_app as app
from decorators import api_wrapper, login_required, WebException
from models import db, Tickets, TicketReplies

import user
import utils

blueprint = Blueprint("tickets", __name__)

@blueprint.route("/create", methods=["POST"])
@login_required
@api_wrapper
def create_ticket():
	params = utils.flat_multi(request.form)

	title = params.get("title")
	body = params.get("body")
	username = session.get("username")

	result = user.get_user(username=username)
	if result is None:
		raise WebException("User does not exist.")

	_user = result.first()

	ticket = Tickets(title, body, _user.uid)
	with app.app_context():
		db.session.add(ticket)
		db.session.commit()

	return { "success": 1, "message": "Ticket created." }

@blueprint.route("/close", methods=["POST"])
@login_required
@api_wrapper
def close_ticket():
	params = utils.flat_multi(request.form)

	htid = params.get("htid")

	result = get_ticket(htid=htid)
	if result is None: raise WebException("Ticket does not exist.") 
	ticket = result.first()
	ticket.opened = False
	with app.app_context():
		db.session.add(ticket)
		db.session.commit()

	return { "success": 1, "message": "Ticket closed." }

@blueprint.route("/open", methods=["POST"])
@login_required
@api_wrapper
def open_ticket():
	params = utils.flat_multi(request.form)

	htid = params.get("htid")

	result = get_ticket(htid=htid)
	if result is None:
		raise WebException("Ticket does not exist.")

	ticket = result.first()
	ticket.opened = True
	with app.app_context():
		db.session.add(ticket)
		db.session.commit()

	return { "success": 1, "message": "Ticket opened." }

@blueprint.route("/data", methods=["GET"])
@login_required
@api_wrapper
def ticket_data():
	opened = []
	closed = []

	params = utils.flat_multi(request.form)

	if params.get("htid") is not None:
		result = get_ticket(htid=htid)
	elif user.is_admin():
		result = get_ticket()
	else:
		result = user.get_user()

		if result is None:
			raise WebException("User does not exist.")

		_user = result.first()

		result = get_ticket(author=_user.uid)

	if result is not None:
		tickets = result.all()
		for ticket in tickets:
			d = {
				"htid": ticket.htid,
				"date": ticket.date,
				"opened": ticket.opened,
				"author": ticket.author,
				"title": ticket.title,
				"body": ticket.body,
				"replies": ticket.get_replies()
			}

			if d["opened"] == True:
				opened.append(d)
			else:
				closed.append(d)

	data = [opened, closed]
	return { "success": 1, "data": data }

def get_ticket(htid=None, author=None, opened=None):
	match = {}
	if htid is not None:
		match.update({ "htid": htid })
	if author is not None:
		match.update({ "author": author })
	if opened is not None:
		match.update({ "opened": opened })
	with app.app_context():
		result = Tickets.query.filter_by(**match)
		return result
