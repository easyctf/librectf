FORGOT_EMAIL_TEMPLATE = open("forgot.mail").read()
REGISTRATION_EMAIL_TEMPLATE = open("registration.mail").read()

USER_LEVELS = ["Administrator", "Student", "Observer", "Teacher"]
USER_REGULAR = 1
USER_OBSERVER = 2
USER_TEACHER = 3

SUPPORTED_LANGUAGES = {
    "cxx": "C++",
    "python2": "Python 2",
    "python3": "Python 3",
    "java": "Java",
}
