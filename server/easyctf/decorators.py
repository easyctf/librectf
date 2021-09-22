from datetime import datetime
from functools import wraps, update_wrapper

from flask import abort, flash, redirect, url_for, session, make_response
from flask_login import current_user, login_required

from easyctf.models import Config


def email_verification_required(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        if not (current_user.is_authenticated and current_user.email_verified):
            session.pop("_flashes", None)
            flash("You need to verify your email first.", "warning")
            return redirect(url_for("users.settings"))
        return func(*args, **kwargs)

    return wrapper


def admin_required(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        if not (current_user.is_authenticated and current_user.admin):
            abort(403)
        return func(*args, **kwargs)

    return wrapper


def teacher_required(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        if not (current_user.is_authenticated and current_user.level == 3):
            abort(403)
        return func(*args, **kwargs)

    return wrapper


def block_before_competition(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        start_time = Config.get("start_time")
        if not current_user.is_authenticated or not (
            current_user.admin
            or (
                start_time
                and current_user.is_authenticated
                and datetime.utcnow() >= datetime.fromtimestamp(int(start_time))
            )
        ):
            abort(403)
        return func(*args, **kwargs)

    return wrapper


def block_after_competition(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        end_time = Config.get("end_time")
        if not current_user.is_authenticated or not (
            current_user.admin
            or (
                end_time
                and current_user.is_authenticated
                and datetime.utcnow() <= datetime.fromtimestamp(int(end_time))
            )
        ):
            abort(403)
        return func(*args, **kwargs)

    return wrapper


def team_required(func):
    @wraps(func)
    @login_required
    def wrapper(*args, **kwargs):
        if not hasattr(current_user, "team") or not current_user.tid:
            flash("You need a team to view this page!", "info")
            return redirect(url_for("teams.create"))
        return func(*args, **kwargs)

    return wrapper


def is_team_captain(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        if not (
            current_user.is_authenticated
            and current_user.tid
            and current_user.team.owner == current_user.uid
        ):
            return abort(403)
        return func(*args, **kwargs)

    return wrapper


def no_cache(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        response = make_response(func(*args, **kwargs))
        response.headers["Last-Modified"] = datetime.now()
        response.headers[
            "Cache-Control"
        ] = "no-store, no-cache, must-revalidate, post-check=0, pre-check=0, max-age=0"
        response.headers["Pragma"] = "no-cache"
        response.headers["Expires"] = "-1"
        return response

    return update_wrapper(wrapper, func)
