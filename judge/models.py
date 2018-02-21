import enum


class Problem:
    def __init__(self, id: int, test_cases: int, time_limit: float, memory_limit: int,
                 generator_code: str, generator_language, grader_code: str, grader_language,
                 source_verifier_code: str=None, source_verifier_language=None):
        self.id = id
        self.test_cases = test_cases
        self.time_limit = time_limit
        self.memory_limit = memory_limit
        self.generator_code = generator_code
        self.generator_language = generator_language
        self.grader_code = grader_code
        self.grader_language = grader_language
        self.source_verifier_code = source_verifier_code
        self.source_verifier_language = source_verifier_language


class Job:
    def __init__(self, id: int, problem: Problem, code: str, language):
        self.id = id
        self.problem = problem
        self.code = code
        self.language = language


class JobVerdict(enum.Enum):
    accepted = 'AC'
    ran = 'RAN'
    invalid_source = 'IS'
    wrong_answer = 'WA'
    time_limit_exceeded = 'TLE'
    memory_limit_exceeded = 'MLE'
    runtime_error = 'RTE'
    illegal_syscall = 'ISC'
    compilation_error = 'CE'
    judge_error = 'JE'


class ExecutionResult:
    def __init__(self, job: Job, verdict: JobVerdict, last_ran_case: int, execution_time: float, execution_memory: int):
        self.job = job
        self.verdict = verdict
        self.last_ran_case = last_ran_case
        self.execution_time = execution_time
        self.execution_memory = execution_memory
