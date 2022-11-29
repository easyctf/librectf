import logging
from io import BytesIO

from flask import Blueprint, abort, flash, redirect, render_template, url_for
from flask_login import current_user, login_required

from easyctf.decorators import is_team_captain, email_verification_required
from easyctf.forms.teams import AddMemberForm, CreateTeamForm, ProfileEditForm
from easyctf.models import Config, Team, User
from easyctf.objects import db
from easyctf.utils import sanitize_avatar, save_file
from easyctf.constants import USER_TEACHER

blueprint = Blueprint("teams", __name__, template_folder="templates")


@blueprint.route("/accept/<int:id>")
@is_team_captain
@login_required
def accept(id):
    return ""


@blueprint.route("/cancel/<int:id>")
@is_team_captain
@login_required
def cancel(id):
    current_team = current_user.team
    target_user = User.get_by_id(id)
    try:
        assert target_user != None, "User not found."
        assert (
            target_user in current_team.outgoing_invitations
        ), "No invitation for this user found."
        current_team.outgoing_invitations.remove(target_user)
        db.session.add(current_team)
        db.session.commit()
        flash(
            "Invitation to %s successfully withdrawn." % target_user.username, "success"
        )
    except AssertionError as e:
        flash(str(e), "danger")
    return redirect(url_for("teams.settings"))


@blueprint.route("/create", methods=["GET", "POST"])
@email_verification_required
@login_required
def create():
    if current_user.tid:
        return redirect(url_for("teams.profile"))
    create_team_form = CreateTeamForm(prefix="create")
    if create_team_form.validate_on_submit():
        new_team = create_team(create_team_form)
        logging.info("Created team '%s' (id=%s)!" % (new_team.teamname, new_team.tid))
        return redirect(url_for("teams.profile"))
    return render_template("teams/create.html", create_team_form=create_team_form)


@blueprint.route("/evict/<int:id>")
@is_team_captain
@login_required
def evict(id):
    current_team = current_user.team
    target_user = User.get_by_id(id)
    try:
        assert target_user != None, "User not found."
        assert target_user in current_team.members, "This user isn't in your team!"
        assert target_user.uid != current_team.owner, "You can't evict the captain!"
        current_team.members.remove(target_user)
        db.session.add(target_user)
        db.session.commit()
        flash("Removed %s from the team." % target_user.username, "success")
    except AssertionError as e:
        flash(str(e), "danger")
    return redirect(url_for("teams.settings"))


@blueprint.route("/profile", methods=["GET", "POST"])
@blueprint.route("/profile/<int:tid>", methods=["GET", "POST"])
def profile(tid=None):
    if tid is None and current_user.is_authenticated:
        if current_user.tid is None:
            return redirect(url_for("teams.create"))
        else:
            return redirect(url_for("teams.profile", tid=current_user.tid))
    team = Team.get_by_id(tid)
    if team is None:
        abort(404)
    if not current_user.is_authenticated:
        flash("Please login to view team profiles!", "warning")
        return redirect(url_for("users.login"))
    if not current_user.admin and (team.banned and current_user.tid != team.tid):
        abort(404)
    return render_template("teams/profile.html", team=team)


@blueprint.route("/settings", methods=["GET", "POST"])
@is_team_captain
@login_required
def settings():
    current_team = current_user.team
    add_member_form = AddMemberForm(prefix="add-member")
    profile_edit_form = ProfileEditForm(prefix="profile-edit")
    if add_member_form.submit.data and add_member_form.validate_on_submit():
        target_user = add_member_form.get_user()
        current_team.outgoing_invitations.append(target_user)
        db.session.add(current_team)
        db.session.commit()
        flash("Invitation to %s sent!" % target_user.username, "info")
        return redirect(url_for("teams.settings"))
    elif profile_edit_form.submit.data and profile_edit_form.validate_on_submit():
        for field in profile_edit_form:
            if field.short_name == "avatar":
                if hasattr(field.data, "read") and len(field.data.read()) > 0:
                    field.data.seek(0)
                    f = BytesIO(field.data.read())
                    new_avatar = sanitize_avatar(f)
                    if new_avatar:
                        response = save_file(
                            new_avatar, prefix="team_avatar", suffix=".png"
                        )
                        if response.status_code == 200:
                            current_team._avatar = response.text
                continue
            if hasattr(current_team, field.short_name):
                setattr(current_team, field.short_name, field.data)
        if profile_edit_form.remove_avatar.data:
            current_team._avatar = None
        db.session.add(current_team)
        db.session.commit()
        flash("Profile updated.", "success")
        return redirect(url_for("teams.settings"))
    else:
        for field in profile_edit_form:
            if hasattr(current_team, field.short_name):
                field.data = getattr(current_team, field.short_name, "")
    return render_template(
        "teams/settings.html",
        team=current_team,
        profile_edit_form=profile_edit_form,
        add_member_form=add_member_form,
    )


def create_team(form):
    new_team = Team(owner=current_user.uid)
    db.session.add(new_team)
    db.session.commit()
    current_user.tid = new_team.tid
    form.populate_obj(current_user.team)
    db.session.add(current_user)
    db.session.commit()
    return new_team
