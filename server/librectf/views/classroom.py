from flask import Blueprint, abort, flash, redirect, render_template, url_for
from flask_login import current_user, login_required
from sqlalchemy import func

from easyctf.decorators import teacher_required, team_required
from easyctf.forms.classroom import AddTeamForm, NewClassroomForm
from easyctf.models import Classroom, Team, classroom_invitation, team_classroom
from easyctf.objects import db

blueprint = Blueprint("classroom", __name__)


@blueprint.route("/")
@team_required
@login_required
def index():
    invites = None
    if current_user.level == 3:
        classes = Classroom.query.filter_by(owner=current_user.uid).all()
    elif not current_user.tid:
        flash("You must be a part of a team to join classes.", "info")
        return redirect(url_for("teams.create"))
    else:
        classes = current_user.team.classrooms
        invites = current_user.team.classroom_invites
    return render_template("classroom/index.html", classes=classes, invites=invites)


@blueprint.route("/new", methods=["GET", "POST"])
@login_required
@teacher_required
def new():
    new_classroom_form = NewClassroomForm()
    if new_classroom_form.validate_on_submit():
        classroom = Classroom(name=new_classroom_form.name.data, owner=current_user.uid)
        db.session.add(classroom)
        db.session.commit()
        flash("Created classroom.", "success")
        return redirect(url_for("classroom.view", id=classroom.id))
    return render_template("classroom/new.html", new_classroom_form=new_classroom_form)


@blueprint.route("/delete/<int:id>")
@login_required
@teacher_required
def delete(id):
    classroom = Classroom.query.filter_by(id=id).first()
    if not classroom:
        abort(404)
    if current_user.uid != classroom.owner:
        abort(403)
    db.session.delete(classroom)
    db.session.commit()
    return redirect(url_for("classroom.index"))


@blueprint.route("/accept/<int:id>")
@login_required
def accept(id):
    invitation = db.session.query(classroom_invitation).filter_by(
        team_id=current_user.tid, classroom_id=id
    )
    if not invitation:
        abort(404)
    classroom = Classroom.query.filter_by(id=id).first()
    if not classroom:
        abort(404)
    classroom.teams.append(current_user.team)
    if current_user.team in classroom.invites:
        classroom.invites.remove(current_user.team)
    db.session.commit()
    flash("Joined classroom.", "success")
    return redirect(url_for("classroom.view", id=id))


@blueprint.route("/remove/<int:cid>/<int:tid>")
@login_required
@teacher_required
def remove(cid, tid):
    classroom = Classroom.query.filter_by(id=cid).first()
    if not classroom:
        abort(404)
    if current_user.uid != classroom.owner:
        abort(403)
    team = Team.query.filter_by(tid=tid).first()
    if not team:
        abort(404)
    if team not in classroom:
        abort(403)
    classroom.teams.remove(team)
    db.session.commit()
    return redirect(url_for("classroom.view", id=cid))


@blueprint.route("/<int:id>", methods=["GET", "POST"])
@login_required
def view(id):
    classroom = Classroom.query.filter_by(id=id).first()
    if not classroom:
        return redirect("classroom.index")
    if not (
        current_user.uid == classroom.owner
        or db.session.query(team_classroom)
        .filter_by(team_id=current_user.tid, classroom_id=classroom.id)
        .count()
    ):
        abort(403)
    add_team_form = AddTeamForm(prefix="addteam")
    if add_team_form.validate_on_submit():
        if current_user.uid != classroom.owner:
            abort(403)
        team = Team.query.filter(
            func.lower(Team.teamname) == add_team_form.name.data.lower()
        ).first()
        classroom.invites.append(team)
        flash("Team invited.", "success")
        db.session.commit()
        return redirect(url_for("classroom.view", id=id))
    users = [user for _team in classroom.teams for user in _team.members]
    return render_template(
        "classroom/view.html",
        classroom=classroom,
        users=users,
        add_team_form=add_team_form,
    )
