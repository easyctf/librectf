import os
import pathlib
from typing import Dict


APP_ROOT = pathlib.Path(os.path.dirname(os.path.abspath(__file__)))
CONFINE_PATH = APP_ROOT / 'confine'

COMPILATION_TIME_LIMIT = 10
GRADER_TIME_LIMIT = 10

PARTIAL_JOB_SUBMIT_TIME_THRESHOLD = 2  # Seconds
PARTIAL_JOB_SUBMIT_CASES_THRESHOLD = 10
