import json
import os
from functools import wraps

from flask import Blueprint, abort, current_app, flash, make_response, render_template, request, url_for
from flask_login import current_user, login_required

from easyctf.decorators import block_before_competition, team_required
from easyctf.forms.chals import ProblemSubmitForm, ProgrammingSubmitForm
from easyctf.forms.game import GameStateUpdateForm
from easyctf.models import AutogenFile, GameState, Job, Problem, Solve, User, WrongFlag
from easyctf.objects import cache, db, sentry

blueprint = Blueprint("game", __name__, template_folder="templates")


def api_view(f):
    @wraps(f)
    def wrapper(*args, **kwargs):
        status, result = f(*args, **kwargs)
        return make_response(json.dumps(result or dict()), status, {"Content-Type": "application/json; charset=utf-8"})
    return wrapper


@blueprint.route("/", methods=["GET"])
@login_required
@team_required
@block_before_competition
def game():
    problem_submit_form = ProblemSubmitForm()

    return render_template("game/game.html", problem_submit_form=problem_submit_form)


@blueprint.route("/problems", methods=["GET"])
@login_required
@team_required
@block_before_competition
@api_view
def problems():
    if current_user.admin:
        problems = Problem.query.order_by(Problem.value).all()
    else:
        problems = current_user.team.get_unlocked_problems()
    formatted_problems = {problem.pid: problem.api_summary() for problem in problems}
    return 200, formatted_problems


@blueprint.route("/submit", methods=["POST"])
@login_required
@team_required
@block_before_competition
@api_view
def submit():
    problem_submit_form = ProblemSubmitForm(**request.get_json())
    if problem_submit_form.validate():
        problem = Problem.query.get_or_404(int(problem_submit_form.pid.data))
        result, message = problem.try_submit(problem_submit_form.flag.data)

        return 200, {
            "result": result,
            "message": message,
        }
    return 400, None  # TODO: actually return validation error


@blueprint.route("/state", methods=["GET", "POST"])
@login_required
@team_required
@block_before_competition
def game_state_get():
    game_state = GameState.query.filter_by(uid=current_user.uid).first()
    if game_state is None:
        game_state = GameState(uid=current_user.uid)
        # TODO: proper upserting
        db.session.add(game_state)
        db.session.commit()
    return make_response(game_state.state, 200, {"Content-Type": "application/json; charset=utf-8"})


@blueprint.route("/state/update", methods=["POST"])
@login_required
@team_required
@block_before_competition
@api_view
def game_state_update():
    game_state_update_form = GameStateUpdateForm(**request.get_json())
    if game_state_update_form.validate():
        state = game_state_update_form.state.data
        game_state = GameState.query.filter_by(uid=current_user.uid).first()
        if game_state is None:
            game_state = GameState(uid=current_user.uid)
            # TODO: proper upserting
            db.session.add(game_state)
        game_state.state = state
        db.session.commit()
        return 200, None
    return 400, None  # TODO: actually return validation error
