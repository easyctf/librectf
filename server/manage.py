from flask_migrate import Migrate, MigrateCommand
from flask_script import Command, Manager, Server

from easyctf import create_app
from easyctf.models import Problem
from easyctf.objects import db

app = create_app()
migrate = Migrate(app, db)

manager = Manager(app)
manager.add_command("db", MigrateCommand)

ServerCommand = Server(host="0.0.0.0", port=8000, use_debugger=True, use_reloader=True)
manager.add_command("runserver", ServerCommand)


class ImportCommand(Command):
    "Import CTF challenges from local repository."

    def __init__(self):
        Command.__init__(self, func=Problem.import_repository)


manager.add_command("import", ImportCommand)

if __name__ == "__main__":
    manager.run()
