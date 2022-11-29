from flask import Blueprint

import admin

blueprint = Blueprint("api", __name__)

blueprint.register_blueprint(admin.blueprint, url_prefix="/admin")
