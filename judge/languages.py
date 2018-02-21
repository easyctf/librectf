import logging
import subprocess
import os
from abc import ABCMeta
from typing import List, Dict

import config
from models import JobVerdict

logger = logging.getLogger(__name__)
logger.setLevel(logging.INFO)
logging.info('Starting up')


class Language(metaclass=ABCMeta):
    @classmethod
    def compile(cls, source_code: str, workdir: str, executable_name: str, time_limit: float = config.COMPILATION_TIME_LIMIT) -> str:
        raise NotImplementedError()

    @classmethod
    def get_command(cls, workdir: str, executable_name: str) -> List[str]:
        raise NotImplementedError()

    @classmethod
    def get_allowed_files(cls, workdir: str, executable_name: str):
        raise NotImplementedError()

    @classmethod
    def get_allowed_file_prefixes(cls, workdir: str, executable_name: str):
        raise NotImplementedError()


class CXX(Language):
    @classmethod
    def compile(cls, source_code: str, workdir: str, executable_name: str, time_limit: float = config.COMPILATION_TIME_LIMIT) -> str:
        source_file_path = os.path.join(workdir, 'source.cpp')
        with open(source_file_path, 'wb') as source_file:
            source_file.write(source_code.encode('utf-8'))

        executable_file_path = os.path.join(workdir, executable_name)
        try:
            subprocess.check_call(['g++', '--std=c++1y', '-o', executable_file_path, source_file_path],
                                  timeout=config.COMPILATION_TIME_LIMIT)
        except (subprocess.CalledProcessError, subprocess.TimeoutExpired):
            return None

        return executable_name

    @classmethod
    def get_command(cls, workdir: str, executable_name: str) -> List[str]:
        return [os.path.join(workdir, executable_name)]

    @classmethod
    def get_allowed_files(cls, workdir: str, executable_name: str):
        return []

    @classmethod
    def get_allowed_file_prefixes(cls, workdir: str, executable_name: str):
        return []


class Python(Language):
    language_name = 'python'
    interpreter_name = 'python'
    
    @classmethod
    def compile(cls, source_code: str, workdir: str, executable_name: str, time_limit: float = config.COMPILATION_TIME_LIMIT) -> str:
        executable_name += '.py'
        executable_path = os.path.join(workdir, executable_name)
        with open(executable_path, 'wb') as executable_file:
            executable_file.write(source_code.encode('utf-8'))

        """try:
            subprocess.check_call([cls.interpreter_name, '-m', 'py_compile', executable_name],
                                  timeout=config.COMPILATION_TIME_LIMIT)
        except (subprocess.CalledProcessError, subprocess.TimeoutExpired):
            return None"""

        return executable_name

    @classmethod
    def get_command(cls, workdir: str, executable_name: str) -> List[str]:
        return [os.path.join('/usr/bin', cls.interpreter_name), '-s', '-S', os.path.join(workdir, executable_name)]

    @classmethod
    def get_allowed_files(cls, workdir: str, executable_name: str):
        return [
            '/etc/nsswitch.conf',
            '/etc/passwd',
            '/dev/urandom',  # TODO: come up with random policy
            '/tmp',
            '/bin/Modules/Setup',
            workdir,
            os.path.join(workdir, executable_name),
        ]

    @classmethod
    def get_allowed_file_prefixes(cls, workdir: str, executable_name: str):
        return []


class Java(Language):
    @classmethod
    def compile(cls, source_code: str, workdir: str, executable_name: str, time_limit: float = config.COMPILATION_TIME_LIMIT) -> str:
        source_file_path = os.path.join(workdir, 'Main.java')
        with open(source_file_path, 'wb') as source_file:
            source_file.write(source_code.encode('utf-8'))

        executable_file_path = os.path.join(workdir, 'Main')
        try:
            subprocess.check_call(['javac', '-d', workdir, source_file_path],
                                  timeout=config.COMPILATION_TIME_LIMIT)
        except (subprocess.CalledProcessError, subprocess.TimeoutExpired):
            return None

        return 'Main'

    @classmethod
    def get_command(cls, workdir: str, executable_name: str) -> List[str]:
        return ['/usr/bin/java', '-XX:-UsePerfData', '-XX:+DisableAttachMechanism', '-Xmx256m', '-Xrs', '-cp',
                workdir, executable_name]

    @classmethod
    def get_allowed_files(cls, workdir: str, executable_name: str):
        return [
            '/etc/nsswitch.conf',
            '/etc/passwd',
            '/tmp',
            workdir,
            os.path.join(workdir, executable_name + '.class'),
        ]

    @classmethod
    def get_allowed_file_prefixes(cls, workdir: str, executable_name: str):
        return [
            '/etc/java-7-openjdk/',
            '/tmp/.java_pid',
            '/tmp/',
        ]


class Python2(Python):
    language_name = 'python2'
    interpreter_name = 'python2.7'


class Python3(Python):
    language_name = 'python3'
    interpreter_name = 'python3.5'


languages = {
    'cxx': CXX,
    'python2': Python2,
    'python3': Python3,
    'java': Java,
}  # type: Dict[str, Language]
