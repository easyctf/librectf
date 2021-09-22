from flask import Blueprint, abort, flash, redirect, render_template, request, url_for

from librectf.decorators import admin_required
from librectf.forms.admin import ProblemForm, SettingsForm
from librectf.models import AutogenFile, Config, Problem, JudgeKey
from librectf.objects import db

blueprint = Blueprint("admin", __name__, template_folder="templates")

DEFAULT_GRADER = """def grade(random, submission):
    if "correct_flag" in submission:
        return True, "Nice!"
    return False, "Nope."
"""


@blueprint.route("/problems/<int:pid>/delete", methods=["POST"])
@admin_required
def delete_problem(pid):
    problem = Problem.get_by_id(pid)
    if problem is None:
        abort(404)
    db.session.delete(problem)
    db.session.commit()
    flash("Problem {} has been deleted!".format(repr(problem.name)), "info")
    return redirect(url_for("admin.problems"))


@blueprint.route("/problems", methods=["GET", "POST"])
@blueprint.route("/problems/<int:pid>", methods=["GET", "POST"])
@admin_required
def problems(pid=None):
    problem = None
    problem_form = ProblemForm()
    if problem_form.validate_on_submit():
        new_problem = False
        if pid is None:
            p = Problem.query.filter_by(name=problem_form.name.data).first()
            if p:
                flash("Please choose a unique name for this problem.", "warning")
                return redirect(url_for("admin.problems"))
            new_problem = True
            problem = Problem()
        else:
            problem = Problem.get_by_id(pid)
            if problem is None:
                abort(404)
        problem_form.populate_obj(problem)
        db.session.add(problem)
        db.session.flush()
        autogen_files = AutogenFile.query.filter_by(pid=pid)
        if autogen_files.count():
            autogen_files.delete()
        db.session.commit()
        if new_problem:
            flash("Problem {} has been created!".format(repr(problem.name)), "info")
        return redirect(url_for("admin.problems", pid=problem.pid))
    problems = Problem.query.order_by(Problem.value).all()
    if pid is not None:
        problem = Problem.get_by_id(pid)
        if not problem:
            return abort(404)
        if request.method != "POST":
            problem_form = ProblemForm(obj=problem)
            # if problem.programming:
            #     judge_problem = judge_api.problems_get(pid)
            #     if judge_problem.status_code == 404:
            #         abort(500)
            #     problem_form.grader.data = judge_problem.data['grader_code']
            #     problem_form.generator.data = judge_problem.data['generator_code']
    else:
        problem_form.grader.data = DEFAULT_GRADER
    return render_template(
        "admin/problems.html",
        current_problem=problem,
        problems=problems,
        problem_form=problem_form,
    )


@blueprint.route("/settings/judge/key")
@admin_required
def judge_key():
    key = JudgeKey()
    db.session.add(key)
    db.session.commit()
    flash("Key created: {}. This won't be shown again.".format(key.key), "success")
    return redirect(url_for("admin.settings"))


@blueprint.route("/settings", methods=["GET", "POST"])
@admin_required
def settings():
    settings_form = SettingsForm()
    if settings_form.validate_on_submit():
        configs = dict()
        for field in settings_form:
            if field.short_name in ["csrf_token", "submit"]:
                continue
            configs.update(**{field.short_name: field.data})
        Config.set_many(configs)
        flash("CTF settings updated!", "success")
        return redirect(url_for("admin.settings"))
    else:
        configs = Config.get_many([field.short_name for field in settings_form])
        for field in settings_form:
            if field.short_name == "csrf_token":
                continue
            if field.short_name in configs:
                field.data = configs.get(field.short_name, "")
    return render_template("admin/settings.html", settings_form=settings_form)
