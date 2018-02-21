import logging
import os
import shutil
import signal
import sys
import time
import tempfile
import traceback

import executor
from api import API
from models import Job

logger = logging.getLogger(__name__)
logger.setLevel(logging.INFO)
logging.info('Starting up')


api = None
judge_url = None
current_job = None  # type: Job


def loop():
    global current_job
    job = api.claim()
    current_job = job
    if not job:
        logger.debug('No jobs available.')
        return False
    logger.info('Got job %d.', job.id)

    tempdir = tempfile.mkdtemp(prefix='jury-')
    try:
        for execution_result in executor.run_job(job, tempdir):
            # execution_result is partial here

            logger.info('Job %d partially judged; case: %d, time: %.2f, memory: %d',
                        job.id, execution_result.last_ran_case, execution_result.execution_time,
                        execution_result.execution_memory)

            if execution_result.verdict:
                # This should be the last value returned by run_job
                logger.info('Job %d finished with verdict %s.' % (job.id, execution_result.verdict.value))

            if api.submit(execution_result):
                logger.info('Job %d successfully partially submitted.' % job.id)
            else:
                logger.info('Job %d failed to partially submit.' % job.id)
    except:
        traceback.print_exc(file=sys.stderr)
        shutil.rmtree(tempdir, ignore_errors=True)
    finally:
        shutil.rmtree(tempdir, ignore_errors=True)

    return True


if __name__ == '__main__':
    api_key = os.getenv("API_KEY")
    if not api_key:
        print("no api key", file=sys.stderr)
        sys.exit(1)
    judge_url = os.getenv("JUDGE_URL")
    if not judge_url:
        print("no judge url", file=sys.stderr)
        sys.exit(1)
    api = API(api_key, judge_url)
    while True:
        try:
            if not loop():
                time.sleep(3)
        except KeyboardInterrupt:
            sys.exit(0)
        except:
            traceback.print_exc(file=sys.stderr)
