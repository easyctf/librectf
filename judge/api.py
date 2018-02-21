import logging

import requests

from languages import languages, Python3
from models import Job, Problem


class API(object):
    def __init__(self, key, base_url):
        self.key = key
        self.base_url = base_url

    def api_call(self, url, method="GET", data=None, headers=None):
        if headers is None:
            headers = dict()
        headers.update({"API-Key": self.key})
        r = requests.request(method, url, data=data, headers=headers)
        return r

    def claim(self):
        r = self.api_call(self.base_url + "/jobs")
        print("text:", repr(r.text))
        if not r.text:
            return None
        required_fields = ["id", "language", "source", "pid", "test_cases", "time_limit", "memory_limit", "generator_code", "grader_code", "source_verifier_code"]
        # create job object
        obj = r.json()
        if not all(field in obj for field in required_fields):
            return None
        problem = Problem(obj["pid"], obj["test_cases"], obj["time_limit"], obj["memory_limit"],
                          obj["generator_code"], Python3,
                          obj["grader_code"], Python3,
                          obj["source_verifier_code"], Python3)
        language = languages.get(obj["language"])
        if not language:
            return None  # TODO: should definitely not do this
        return Job(obj["id"], problem, obj["source"], language)

    def submit(self, result):
        verdict = result.verdict
        data = dict(
            id=result.job.id,
            verdict=result.verdict.value if verdict else "JE",
            last_ran_case=result.last_ran_case,
            execution_time=result.execution_time,
            execution_memory=result.execution_memory
        )
        r = self.api_call(self.base_url + "/jobs", method="POST", data=data)
        return r.status_code // 100 == 2
