import json
import logging
import os
import shutil
import subprocess
import tempfile
import time
from functools import wraps
from typing import Iterator

import config
from languages import Language
from models import ExecutionResult, Job, JobVerdict, Problem

logger = logging.getLogger(__name__)
logger.setLevel(logging.INFO)
logging.info("Starting up")

verdict_map = {
    "InternalError": JobVerdict.judge_error,
    "RuntimeError": JobVerdict.runtime_error,
    "TimeLimitExceeded": JobVerdict.time_limit_exceeded,
    "MemoryLimitExceeded": JobVerdict.memory_limit_exceeded,
    "IllegalSyscall": JobVerdict.illegal_syscall,
    "IllegalOpen": JobVerdict.illegal_syscall,
    "IllegalWrite": JobVerdict.illegal_syscall,
}


class ExecutionReport:
    def __init__(
        self,
        execution_ok: bool,
        execution_error_code: JobVerdict,
        exitcode: int,
        realtime: float,
        cputime: float,
        memory: int,
    ):
        self.execution_ok = execution_ok
        self.execution_error_code = execution_error_code
        self.exitcode = exitcode
        self.realtime = realtime
        self.cputime = cputime
        self.memory = memory

    @classmethod
    def error_report(cls):
        return cls(
            execution_ok=False,
            execution_error_code=JobVerdict.judge_error,
            exitcode=-1,
            realtime=0,
            cputime=0,
            memory=0,
        )

    @classmethod
    def from_json(cls, json_string: str):
        try:
            obj = json.loads(json_string)
            return cls(
                execution_ok=obj["execution_ok"],
                execution_error_code=verdict_map[obj["execution_error_code"]["code"]]
                if obj["execution_error_code"]
                else None,
                exitcode=obj["exitcode"],
                realtime=obj["realtime"],
                cputime=obj["cputime"],
                memory=obj["memory"],
            )
        except (json.JSONDecodeError, KeyError):
            logger.error("Failed to load execution report from json!")
            return cls.error_report()


class ExecutionProfile:
    def __init__(
        self,
        confine_path: str,
        problem: Problem,
        language: Language,
        workdir: str,
        input_file="input",
        output_file="output",
        error_file="error",
        report_file="report",
    ):
        self.confine_path = confine_path
        self.language = language
        self.workdir = workdir
        self.time_limit = problem.time_limit
        self.memory_limit = problem.memory_limit
        self.input_file = os.path.join(workdir, input_file)
        self.output_file = os.path.join(workdir, output_file)
        self.error_file = os.path.join(workdir, error_file)
        self.report_file = os.path.join(workdir, report_file)

    def as_json(self, executable_name: str):
        return json.dumps(
            {
                "cputime_limit": self.time_limit,
                "realtime_limit": self.time_limit * 1000,
                "allowed_files": self.language.get_allowed_files(
                    self.workdir, executable_name
                ),
                "allowed_prefixes": self.language.get_allowed_file_prefixes(
                    self.workdir, executable_name
                ),
                "stdin_file": self.input_file,
                "stdout_file": self.output_file,
                "stderr_file": self.error_file,
                "json_report_file": self.report_file,
            }
        )

    def execute(self, executable_name: str) -> ExecutionReport:
        return Executor(self).execute(executable_name)


class Executor:
    def __init__(self, profile: ExecutionProfile):
        self.profile = profile

    def execute(self, executable_name: str) -> ExecutionReport:
        command = self.profile.language.get_command(
            self.profile.workdir, executable_name
        )

        config_file_path = os.path.join(self.profile.workdir, "confine.json")
        with open(config_file_path, "w") as config_file:
            config_file.write(self.profile.as_json(executable_name))

        try:
            subprocess.check_call(
                [self.profile.confine_path, "-c", config_file_path, "--", *command],
                timeout=self.profile.time_limit * 2,
            )
        except (subprocess.CalledProcessError, subprocess.TimeoutExpired):
            return ExecutionReport.error_report()

        with open(
            os.path.join(self.profile.workdir, self.profile.report_file)
        ) as report_file:
            execution_report = ExecutionReport.from_json(report_file.read())
        return execution_report


def use_tempdir(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        before_dir = os.getcwd()
        tempdir = tempfile.mkdtemp(prefix="jury-")
        os.chdir(tempdir)

        result = func(*args, tempdir=tempdir, **kwargs)

        os.chdir(before_dir)
        # shutil.rmtree(tempdir)
        return result

    return wrapper


# @use_tempdir
def run_job(job: Job, tempdir: str) -> Iterator[ExecutionResult]:
    result = ExecutionResult(
        job=job,
        verdict=JobVerdict.judge_error,
        last_ran_case=0,
        execution_time=0,
        execution_memory=0,
    )

    if job.problem.source_verifier_code and job.problem.source_verifier_language:
        source_verifier_executable = job.problem.source_verifier_language.compile(
            job.problem.source_verifier_code, tempdir, "source_verifier"
        )
        if not source_verifier_executable:
            logger.error(
                "Source verifier failed to compile for problem %d" % job.problem.id
            )
            result.verdict = JobVerdict.judge_error
            yield result
            return

        with open(os.path.join(tempdir, "source"), "wb") as source_file:
            source_file.write(job.code.encode("utf-8"))

        execution_profile = ExecutionProfile(
            confine_path=str(config.CONFINE_PATH),
            problem=job.problem,
            language=job.problem.source_verifier_language,
            workdir=tempdir,
            input_file="source",
            output_file="source_verifier_result",
        )

        execution_result = execution_profile.execute(source_verifier_executable)

        if not execution_result.execution_ok:
            result.verdict = JobVerdict.judge_error
            yield result
            return

        with open(
            os.path.join(tempdir, "source_verifier_result")
        ) as source_verifier_result_file:
            if source_verifier_result_file.read().strip() != "OK":
                result.verdict = JobVerdict.invalid_source
                yield result
                return

    program_executable = job.language.compile(job.code, tempdir, "program")
    if not program_executable:
        result.verdict = JobVerdict.compilation_error
        yield result
        return

    generator_executable = job.problem.generator_language.compile(
        job.problem.generator_code, tempdir, "generator"
    )
    if not generator_executable:
        logger.error("Generator failed to compile for problem %d" % job.problem.id)
        result.verdict = JobVerdict.judge_error
        yield result
        return

    grader_executable = job.problem.grader_language.compile(
        job.problem.grader_code, tempdir, "grader"
    )
    if not grader_executable:
        logger.error("Grader failed to compile for problem %d" % job.problem.id)
        result.verdict = JobVerdict.judge_error
        yield result
        return

    result.verdict = None
    last_submitted_time = time.time()
    last_submitted_case = 0
    for case_number in range(1, job.problem.test_cases + 1):
        result.last_ran_case = case_number
        case_result = run_test_case(
            job,
            case_number,
            tempdir,
            program_executable,
            generator_executable,
            grader_executable,
        )
        if case_result.verdict != JobVerdict.accepted:
            result = case_result
            break
        result.execution_time = max(result.execution_time, case_result.execution_time)
        result.execution_memory = max(
            result.execution_memory, case_result.execution_memory
        )

        # Yield result if over threshold and is not last case
        # If verdict calculation takes time, result should be changed to yield even if is last case.
        if (
            time.time() - last_submitted_time > config.PARTIAL_JOB_SUBMIT_TIME_THRESHOLD
            or case_number - last_submitted_case
            > config.PARTIAL_JOB_SUBMIT_CASES_THRESHOLD
        ) and case_number != job.problem.test_cases + 1:
            yield result

            # We want to let the programs run for `threshold` time before another potential pause
            last_submitted_time = time.time()
            last_submitted_case = case_number

    if not result.verdict:
        result.verdict = JobVerdict.accepted

    yield result


def run_test_case(
    job: Job,
    case_number: int,
    workdir: str,
    program_executable: str,
    generator_executable: str,
    grader_executable: str,
) -> ExecutionResult:
    result = ExecutionResult(
        job=job,
        verdict=JobVerdict.judge_error,
        last_ran_case=case_number,
        execution_time=0,
        execution_memory=0,
    )

    with open(os.path.join(workdir, "case_number"), "wb") as case_number_file:
        case_number_file.write(str(case_number).encode("utf-8"))

    generator_execution_profile = ExecutionProfile(
        confine_path=str(config.CONFINE_PATH),
        problem=job.problem,
        language=job.problem.generator_language,
        workdir=workdir,
        input_file="case_number",
        output_file="input",
    )
    generator_result = generator_execution_profile.execute(generator_executable)

    if not generator_result.execution_ok:
        logger.error(
            "Generator failed for test case %d of problem %d with error %s"
            % (case_number, job.problem.id, generator_result.execution_error_code)
        )
        return result

    program_execution_profile = ExecutionProfile(
        confine_path=str(config.CONFINE_PATH),
        problem=job.problem,
        language=job.language,
        workdir=workdir,
        input_file="input",
        output_file="program_output",
        error_file="program_error",
    )
    execution_result = program_execution_profile.execute(program_executable)

    result.execution_time = execution_result.realtime
    result.execution_memory = execution_result.memory
    if not execution_result.execution_ok:
        result.verdict = execution_result.execution_error_code
        return result

    grader_execution_profile = ExecutionProfile(
        confine_path=str(config.CONFINE_PATH),
        problem=job.problem,
        language=job.problem.grader_language,
        workdir=workdir,
        input_file="input",
        output_file="grader_output",
        error_file="grader_error",
    )
    grader_result = grader_execution_profile.execute(grader_executable)

    if not grader_result.execution_ok:
        logger.error(
            "Grader failed for test case %d of problem %d with error %s"
            % (case_number, job.problem.id, grader_result.execution_error_code)
        )
        result.verdict = JobVerdict.judge_error
        return result

    with open(os.path.join(workdir, "program_output"), "rb") as program_output, open(
        os.path.join(workdir, "grader_output"), "rb"
    ) as grader_output:
        if program_output.read().strip() == grader_output.read().strip():
            final_verdict = JobVerdict.accepted
        else:
            final_verdict = JobVerdict.wrong_answer

    result.verdict = final_verdict

    return result
