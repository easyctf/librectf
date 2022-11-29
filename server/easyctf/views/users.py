import json
from datetime import datetime, timedelta
from io import BytesIO
from string import Template

import pyqrcode
from flask import (Blueprint, abort, flash, redirect, render_template, request,
                   url_for)
from flask_login import current_user, login_required, login_user, logout_user
from sqlalchemy import func

from easyctf.constants import (FORGOT_EMAIL_TEMPLATE,
                               REGISTRATION_EMAIL_TEMPLATE, USER_LEVELS)
from easyctf.forms.users import (ChangeLoginForm, LoginForm,
                                 PasswordForgotForm, PasswordResetForm,
                                 ProfileEditForm, RegisterForm,
                                 TwoFactorAuthSetupForm)
from easyctf.models import Config, PasswordResetToken, Team, User
from easyctf.objects import db, sentry
from easyctf.utils import (generate_string, get_redirect_target, redirect_back,
                           sanitize_avatar, save_file, send_mail)

blueprint = Blueprint("users", __name__, template_folder="templates")


@blueprint.route("/accept/<int:id>")
@login_required
def accept(id):
    target_team = Team.get_by_id(id)
    max_size = Config.get_team_size()
    try:
        assert not current_user.tid, "You're already in a team!"
        assert current_user in target_team.outgoing_invitations, "There is no invitation for you!"
        if not target_team.admin:
            assert target_team.size < max_size, "This team has already reached the maximum member limit!"
        target_team.outgoing_invitations.remove(current_user)
        target_team.members.append(current_user)
        db.session.add(current_user)
        db.session.add(target_team)
        db.session.commit()
        flash("Successfully joined team %s!" % target_team.teamname, "success")
        return redirect(url_for("teams.profile"))
    except AssertionError as e:
        flash(str(e), "danger")
    return redirect(url_for("teams.create"))


@blueprint.route("/password/forgot", methods=["GET", "POST"])
def forgot():
    forgot_form = PasswordForgotForm()
    if forgot_form.validate_on_submit():
        if forgot_form.user is not None:
            token = PasswordResetToken(active=True, uid=forgot_form.user.uid, email=forgot_form.email.data, expire=datetime.utcnow() + timedelta(days=1))
            db.session.add(token)
            db.session.commit()
            url = url_for("users.reset", code=token.token, _external=True)
            # TODO: stick this into the template
            send_mail(forgot_form.email.data, "%s Password Reset" % Config.get("ctf_name"), "Click here to reset your password: %s" % url)
            flash("Sent! Check your email.", "success")
        return redirect(url_for("users.forgot"))
    return render_template("users/forgot.html", forgot_form=forgot_form)


@blueprint.route("/password/reset/<string:code>", methods=["GET", "POST"])
def reset(code):
    token = PasswordResetToken.query.filter_by(token=code, active=True).first()
    if (not token) or token.expired or token.email != token.user.email:
        return redirect(url_for("base.index"))
    reset_form = PasswordResetForm()
    if reset_form.validate_on_submit():
        user = User.get_by_id(token.uid)
        user.password = reset_form.password.data
        token.active = False
        db.session.add(user)
        db.session.add(token)
        db.session.commit()
        flash("Password has been reset! Try logging in now.", "success")
        return redirect(url_for("users.login", next=url_for("users.profile")))
    return render_template("users/reset.html", reset_form=reset_form)


@blueprint.route("/login", methods=["GET", "POST"])
def login():
    if current_user.is_authenticated:
        return redirect(url_for("users.profile", uid=current_user.uid))
    login_form = LoginForm(prefix="login")
    next = get_redirect_target()
    if login_form.validate_on_submit():
        target_user = login_form.get_user()
        if target_user.otp_confirmed and not target_user.verify_totp(login_form.code.data):
            flash("Invalid code.", "danger")
            return render_template("users/login.html", login_form=login_form, next=next)

        login_user(target_user, remember=login_form.remember.data)
        flash("Successfully logged in as %s!" % target_user.username, "success")
        if sentry.client:
            sentry.client.capture_breadcrumb(message="login", category="user:login", level="info", data=dict(uid=target_user.uid, username=target_user.username), timestamp=datetime.now())
        return redirect_back("users.profile")
    return render_template("users/login.html", login_form=login_form, next=next)


@blueprint.route("/logout")
def logout():
    logout_user()
    return redirect(url_for("base.index"))


@blueprint.route("/profile")
@blueprint.route("/profile/<int:uid>")
def profile(uid=None):
    if uid is None and current_user.is_authenticated:
        return redirect(url_for("users.profile", uid=current_user.uid))
    user = User.get_by_id(uid)
    if user is None:
        abort(404)
    if not current_user.is_authenticated:
        flash("Please login to view user profiles!", "warning")
        return redirect(url_for("users.login"))
    user.type = USER_LEVELS[user.level]
    return render_template("users/profile.html", user=user)


@blueprint.route("/register", methods=["GET", "POST"])
def register():
    if current_user.is_authenticated:
        return redirect(url_for("users.profile", uid=current_user.uid))
    register_form = RegisterForm(prefix="register")
    if register_form.validate_on_submit():
        new_user = register_user(register_form.name.data,
                                 register_form.email.data,
                                 register_form.username.data,
                                 register_form.password.data,
                                 int(register_form.level.data), admin=False)
        login_user(new_user)
        return redirect(url_for("users.profile"))
    return render_template("users/register.html", register_form=register_form)


@blueprint.route("/settings", methods=["GET", "POST"])
@login_required
def settings():
    change_login_form = ChangeLoginForm(prefix="change-password")
    profile_edit_form = ProfileEditForm(prefix="profile-edit")
    if change_login_form.validate_on_submit() and change_login_form.submit.data:
        current_email = current_user.email
        if change_login_form.old_password.data:
            if change_login_form.email.data != current_email:
                # flash("changed email")
                current_user.email = change_login_form.email.data
                current_user.email_verified = False
                current_user.email_token = generate_string()
            if change_login_form.password.data:
                # flash("changed password")
                current_user.password = change_login_form.password.data
        db.session.add(current_user)
        db.session.commit()
        flash("Login info updated.", "success")
        return redirect(url_for("users.settings"))
    elif profile_edit_form.validate_on_submit() and profile_edit_form.submit.data:
        for field in profile_edit_form:
            if field.short_name == "avatar":
                if hasattr(field.data, "read") and len(field.data.read()) > 0:
                    field.data.seek(0)
                    f = BytesIO(field.data.read())
                    new_avatar = sanitize_avatar(f)
                    if new_avatar:
                        response = save_file(new_avatar, prefix="user_avatar", suffix=".png")
                        if response.status_code == 200:
                            current_user._avatar = response.text
                continue
            if hasattr(current_user, field.short_name):
                setattr(current_user, field.short_name, field.data)
        if profile_edit_form.remove_avatar.data:
            current_user._avatar = None
        db.session.add(current_user)
        db.session.commit()
        flash("Profile updated.", "success")
        return redirect(url_for("users.settings"))
    else:
        change_login_form.email.data = current_user.email
        for field in profile_edit_form:
            if hasattr(current_user, field.short_name):
                field.data = getattr(current_user, field.short_name, "")
    return render_template("users/settings.html", change_login_form=change_login_form, profile_edit_form=profile_edit_form)


@blueprint.route("/two_factor/required")
def two_factor_required():
    user = User.query.filter(
        func.lower(User.username) == request.args.get("username", "").lower()
    ).first()
    if not user:
        return json.dumps(False)
    return json.dumps(user.otp_confirmed)


@blueprint.route("/two_factor/setup", methods=["GET", "POST"])
@login_required
def two_factor_setup():
    two_factor_form = TwoFactorAuthSetupForm()
    if two_factor_form.validate_on_submit():
        current_user.otp_confirmed = True
        db.session.add(current_user)
        db.session.commit()
        flash("Two-factor authentication setup is complete.", "success")
        return redirect(url_for("users.settings"))
    return render_template("users/two_factor/setup.html", two_factor_form=two_factor_form)


@blueprint.route("/two_factor/qr")
@login_required
def two_factor_qr():
    url = pyqrcode.create(current_user.get_totp_uri())
    stream = BytesIO()
    url.svg(stream, scale=6)
    return stream.getvalue(), 200, {
        "Content-Type": "image/svg+xml",
        "Cache-Control": "no-cache, no-store, must-revalidate",
        "Pragma": "no-cache",
        "Expires": 0,
        "Secret": current_user.otp_secret
    }


@blueprint.route("/two_factor/disable")
@login_required
def two_factor_disable():
    current_user.otp_confirmed = False
    db.session.add(current_user)
    db.session.commit()
    flash("Two-factor authentication disabled.", "success")
    return redirect(url_for("users.settings"))


@blueprint.route("/verify/<string:code>")
@login_required
def verify(code):
    if current_user.email_verified:
        flash("You've already verified your email.", "info")
    elif current_user.email_token == code:
        current_user.email_verified = True
        db.session.add(current_user)
        db.session.commit()
        flash("Email verified!", "success")
    else:
        flash("Incorrect code.", "danger")
    return redirect(url_for("users.settings"))


@blueprint.route("/verify")
@login_required
def verify_email():
    if current_user.email_verified:
        return ""
    code = generate_string()
    current_user.email_token = code
    db.session.add(current_user)
    db.session.commit()
    try:
        link = url_for("users.verify", code=code, _external=True)
        response = send_verification_email(current_user.username, current_user.email, link)
        if response.status_code // 100 != 2:
            return "failed"
        return "success"
    except Exception as e:
        return str(e)


def send_verification_email(username, email, verification_link):
    subject = "[ACTION REQUIRED] EasyCTF Email Verification"
    body = Template(REGISTRATION_EMAIL_TEMPLATE).substitute({
        "link": verification_link,
        "username": username
    })
    return send_mail(email, subject, body)


def register_user(name, email, username, password, level, admin=False, **kwargs):
    new_user = User(name=name, username=username, password=password, email=email, level=level, admin=admin)
    for key, value in list(kwargs.items()):
        setattr(new_user, key, value)
    code = generate_string()
    new_user.email_token = code

    # TODO: Config for this
    # send_verification_email(username, email, url_for("users.verify", code=code, _external=True))

    db.session.add(new_user)
    db.session.commit()
    return new_user
