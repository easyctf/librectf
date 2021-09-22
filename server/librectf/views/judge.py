import enum
import json
import traceback
from datetime import datetime, timedelta
from functools import wraps

from flask import Blueprint, abort, make_response, request

from sqlalchemy import and_, or_
from librectf.models import Job, JudgeKey, Solve, Problem
from librectf.objects import db

blueprint = Blueprint("judge", __name__)


def api_view(f):
    @wraps(f)
    def wrapper(*args, **kwargs):
        api_key = request.headers.get("API-Key")
        if not api_key:
            return abort(403)
        key = JudgeKey.query.filter_by(key=api_key).first()
        if not key:
            return abort(403)
        status, result = f(*args, **kwargs)
        return make_response(
            json.dumps(result or dict()),
            status,
            {"Content-Type": "application/json; charset=utf-8"},
        )

    return wrapper


@blueprint.route("/jobs", methods=["GET", "POST"])
@api_view
def jobs():
    if request.method == "GET":
        # implement language preference later
        available = (
            Job.query.filter(
                or_(
                    Job.status == 0,
                    and_(
                        Job.status == 1,
                        Job.claimed < datetime.utcnow() - timedelta(minutes=5),
                    ),
                )
            )
            .order_by(Job.submitted)
            .first()
        )
        if not available:
            return 204, []
        # assign job to current judge
        info = dict(
            # job info
            id=available.id,
            language=available.language,
            source=available.contents,
            # problem info
            pid=available.problem.pid,
            test_cases=available.problem.test_cases,
            time_limit=available.problem.time_limit,
            memory_limit=available.problem.memory_limit,
            generator_code=available.problem.generator,
            grader_code=available.problem.grader,
            source_verifier_code=available.problem.source_verifier or "",
        )
        available.status = 1
        available.claimed = datetime.utcnow()
        db.session.add(available)
        db.session.commit()
        return 202, info
    elif request.method == "POST":
        # expect
        id_raw = request.form.get("id")
        print("id = ", id_raw)
        if not (id_raw and id_raw.isdigit()):
            return 400, None
        id = int(id_raw)
        job = Job.query.filter_by(id=id).first()
        if not job:
            return 404, None
        try:
            job.status = 2
            job.verdict = request.form.get("verdict")
            job.execution_time = float(request.form.get("execution_time"))
            job.execution_memory = int(request.form.get("execution_memory"))
            job.completed = datetime.utcnow()
            db.session.add(job)
            if job.verdict == "AC":
                solve = Solve.query.filter_by(pid=job.pid, tid=job.tid).first()
                if not solve:
                    solve = Solve(
                        pid=job.pid, uid=job.uid, tid=job.tid, _date=job.completed
                    )
                    db.session.add(solve)
            db.session.commit()
            return 202, None
        except Exception:
            # failed
            job.status = 3
            job.feedback = traceback.format_exc()
            db.session.add(job)
            db.session.commit()
            return 400, None
