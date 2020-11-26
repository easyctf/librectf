import json
import os

from flask import Blueprint, abort, current_app, flash, redirect, render_template, url_for
from flask_login import current_user, login_required

from easyctf.decorators import block_before_competition, team_required, no_cache
from easyctf.forms.chals import ProblemSubmitForm, ProgrammingSubmitForm
from easyctf.models import AutogenFile, Job, Problem, Solve, User, WrongFlag
from easyctf.objects import cache, db, sentry

blueprint = Blueprint("chals", __name__, template_folder="templates")


@blueprint.route("/list", methods=["GET", "POST"])
@login_required
@team_required
@block_before_competition
def list():
    problem_submit_form = ProblemSubmitForm()
    if problem_submit_form.validate_on_submit():
        problem = Problem.get_by_id(int(problem_submit_form.pid.data))
        if problem is None or not current_user.team.has_unlocked(problem):
            flash("Problem not found.", "info")
            return redirect(url_for("chals.list"))

        result, message = problem.try_submit(problem_submit_form.flag.data)

        if result == "success":
            flash(message, "success")
        elif result == "error":
            flash(message, "info")
        if result == "failure":
            flash(message, "danger")
        return redirect(url_for("chals.list"))
    categories = Problem.categories()
    if current_user.admin:
        problems = Problem.query.filter(Problem.value > 0).order_by(Problem.value).all()
    else:
        problems = current_user.team.get_unlocked_problems()
    return render_template("chals/list.html", categories=categories, problems=problems, problem_submit_form=problem_submit_form)


@blueprint.route("/solves/<int:pid>")
@login_required
@team_required
@block_before_competition
def solves(pid):
    problem = Problem.query.filter_by(pid=pid).first()
    if problem is None:
        abort(404)
    return render_template("chals/solves.html", problem=problem)


@blueprint.route("/shell")
@block_before_competition
@team_required
@login_required
def shell():
    return render_template("chals/shell.html")


@blueprint.route("/shell/credentials")
@block_before_competition
@team_required
@login_required
def shell_credentials():
    team = current_user.team
    credentials = team.credentials()
    if not credentials:
        return abort(500)
    username, password = credentials
    return json.dumps(dict(username=username, password=password))


@blueprint.route("/programming/")
@blueprint.route("/programming/<int:pid>", methods=["GET", "POST"])
@login_required
@team_required
@block_before_competition
def programming(pid=None):
    problems = current_user.team.get_unlocked_problems(programming=True)
    if pid is None:
        if len(problems) == 0:
            return redirect(url_for("chals.list"))
        return redirect(url_for("chals.programming", pid=problems[0].pid))
    problem = Problem.get_by_id(pid)
    if not problem:
        return abort(404)
    if not current_user.team.has_unlocked(problem) or not problem.programming:
        return redirect(url_for("chals.list"))

    programming_submit_form = ProgrammingSubmitForm()
    if programming_submit_form.validate_on_submit():
        if not problem.programming:
            return redirect(url_for("chals.list"))
        job = Job(uid=current_user.uid, tid=current_user.tid, pid=pid,
                  language=programming_submit_form.language.data, contents=programming_submit_form.code.data)
        db.session.add(job)
        db.session.commit()
        flash("Code was sent! Refresh the page for updates.", "success")
        return redirect(url_for("chals.submission", id=job.id))

    return render_template("chals/programming.html", problem=problem,
                           problems=problems, programming_submit_form=programming_submit_form)


@blueprint.route("/programming/status")
@login_required
@team_required
@block_before_competition
def status():
    jobs = Job.query.filter_by(tid=current_user.tid).order_by(Job.submitted.desc()).all()
    return render_template("chals/status.html", jobs=jobs)


@blueprint.route("/programming/submission/<int:id>")
@login_required
@team_required
@block_before_competition
def submission(id):
    job = Job.query.filter_by(id=id).first()
    if not job:
        return abort(404)
    if not current_user.admin and job.tid != current_user.tid:
        return abort(403)
    return render_template("chals/submission.html", problem=job.problem, job=job, user=job.user)


@blueprint.route("/autogen/<int:pid>/<filename>")
@login_required
@team_required
@block_before_competition
@no_cache
def autogen(pid, filename):
    problem = Problem.query.filter_by(pid=pid).first()
    if not problem or not problem.autogen:
        return abort(404)

    tid = current_user.tid
    # If autogen file exists in db, redirect to filestore
    autogen_file = AutogenFile.query.filter_by(pid=pid, tid=tid, filename=filename).first()
    if autogen_file:
        return redirect("{}/{}".format(current_app.config["FILESTORE_STATIC"], autogen_file.url))

    current_path = os.getcwd()
    if problem.path:
        os.chdir(problem.path)
    autogen = problem.get_autogen(tid)
    grader = problem.get_grader()
    generated_problem = grader.generate(autogen)
    if "files" in generated_problem:
        data = generated_problem["files"].get(filename)
        if data is None:
            return abort(404)
        autogen_file = AutogenFile(pid=pid, tid=tid, filename=filename, data=data)
        db.session.add(autogen_file)
        db.session.commit()
        return redirect("{}/{}".format(current_app.config["FILESTORE_STATIC"], autogen_file.url))
    os.chdir(current_path)
    return abort(404)
