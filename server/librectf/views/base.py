from flask import Blueprint, abort, redirect, render_template, request, url_for, flash
from flask_login import current_user, login_required
from sqlalchemy import func

from librectf.models import Egg, WrongEgg, Team, User, EggSolve
from librectf.objects import cache, db

blueprint = Blueprint("base", __name__, template_folder="templates")


@blueprint.route("/")
def index():
    return render_template("base/index.html")


@blueprint.route("/about")
def about():
    return render_template("base/about.html")


@blueprint.route("/rules")
def rules():
    return render_template("base/rules.html")


@blueprint.route("/prizes")
def prizes():
    return render_template("base/prizes.html")


@blueprint.route("/sponsors")
def sponsors():
    return render_template("base/sponsors.html")


@blueprint.route("/team")
# @cache.cached(timeout=0)
def team():
    easyctf_team = db.session.query(User).filter(User.easyctf == True).all()
    return render_template("base/team.html", easyctf_team=easyctf_team)


@blueprint.route("/updates")
def updates():
    return render_template("base/updates.html")


@blueprint.route("/scoreboard")
@login_required
def scoreboard():
    scoreboard = Team.scoreboard()
    return render_template("base/scoreboard.html", scoreboard=scoreboard)


@blueprint.route("/shibboleet", methods=["GET", "POST"])
def easter():
    if not (
        current_user.is_authenticated and (current_user.admin or current_user.team)
    ):
        return abort(404)
    eggs = []
    if request.method == "POST":
        if current_user.admin and request.form.get("submit"):
            newegg_str = request.form.get("egg")
            if newegg_str:
                newegg = Egg(flag=newegg_str)
                db.session.add(newegg)
                db.session.commit()
                flash("New egg has been added!", "success")
        else:
            cand = request.form.get("egg")
            egg = Egg.query.filter_by(flag=cand).first()
            if egg:
                solve = EggSolve.query.filter_by(
                    eid=egg.eid, tid=current_user.tid
                ).first()
                if solve:
                    flash("You already got this one", "info")
                else:
                    solve = EggSolve(
                        eid=egg.eid, tid=current_user.tid, uid=current_user.uid
                    )
                    db.session.add(solve)
                    db.session.commit()
                    flash("Congrats!", "success")
            else:
                submission = WrongEgg.query.filter_by(
                    tid=current_user.tid, submission=cand
                ).first()
                if submission:
                    flash("You've already tried that egg", "info")
                else:
                    submission = WrongEgg(
                        tid=current_user.tid, uid=current_user.uid, submission=cand
                    )
                    db.session.add(submission)
                    db.session.commit()
                    flash("Nope, sorry", "danger")
        return redirect(url_for("base.easter"))
    if current_user.admin:
        eggs = Egg.query.all()
    else:
        eggs = EggSolve.query.filter_by(tid=current_user.tid).all()
    return render_template("base/easter.html", eggs=eggs)
