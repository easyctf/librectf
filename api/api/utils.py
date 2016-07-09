from flask import current_app as app
from functools import wraps
from werkzeug.security import generate_password_hash, check_password_hash

from PIL import Image, ImageDraw
from Crypto.PublicKey import RSA

import datetime
import hashlib
import json
import os
import random
import re
import requests
import string
import traceback
import unicodedata


__check_email_format = lambda email: re.match(".+@.+\..{2,}", email) is not None
__check_ascii = lambda s: all(c in string.printable for c in s)
__check_alphanumeric = lambda s: all(c in string.digits + string.ascii_uppercase + string.ascii_lowercase for c in s)

def isoformat(seconds):
	return datetime.datetime.fromtimestamp(seconds).isoformat() + "Z"

def unix_time_seconds(dt):
	epoch = datetime.datetime.utcfromtimestamp(0)
	return (dt - epoch).total_seconds()

def get_time_since_epoch():
	return unix_time_seconds(datetime.datetime.now())

def hash_password(s):
	return generate_password_hash(s)

def check_password(hashed_password, try_password):
	return check_password_hash(hashed_password, try_password)

def generate_string(length=32, alpha=string.hexdigits):
	return "".join([random.choice(alpha) for x in range(length)])

def flat_multi(multidict):
	flat = {}
	for key, values in multidict.items():
		value = values[0] if type(values) == list and len(values) == 1 else values
		flat[key] = value.encode("utf-8")
	return flat

def send_email(recipient, subject, body):
	with app.app_context():
		api_key = app.config["MAILGUN_KEY"]
		data = {"from": app.config["ADMIN_EMAIL"],
				"to": recipient,
				"subject": subject,
				"text": body
				}
		auth = ("api", api_key)
		return requests.post(app.config["MAILGUN_URL"] + "/messages", auth=auth, data=data)

def generate_identicon(email, filename):
	email = email.strip().lower()
	h = hashlib.sha1(email).hexdigest()
	size = 256
	margin = 0.08
	baseMargin = int(size * margin)
	cell = int((size - baseMargin * 2.0) / 5)
	margin = int((size - cell * 5.0) / 2)
	image = Image.new("RGB", (size, size))
	draw = ImageDraw.Draw(image)

	def hsl2rgb(h, s, b):
		h *= 6
		s1 = []
		s *= b if b < 0.5 else 1-b
		b += s
		s1.append(b)
		s1.append(b - h % 1 * s * 2)
		s *= 2
		b -= s
		s1.append(b)
		s1.append(b)
		s1.append(b + h % 1 * s)
		s1.append(b + s)

		return [
			s1[~~h % 6], s1[(h|16) % 6], s1[(h|8) % 6]
		]

	rgb = hsl2rgb(int(h[-7:], 16) & 0xfffffff, 0.5, 0.7)
	bg = (255, 255, 255)
	fg = (int(rgb[0] * 255), int(rgb[1] * 255), int(rgb[2] * 255))

	# print fg, bg
	draw.rectangle([(0, 0), (size, size)], fill=bg)

	for i in range(15):
		c = bg if int(h[i], 16) % 2 == 1 else fg
		if i < 5:
			draw.rectangle([(2*cell + margin, i*cell + margin), (3*cell + margin, (i+1)*cell + margin)], fill=c)
		elif i < 10:
			draw.rectangle([(1*cell + margin, (i-5)*cell + margin), (2*cell + margin, (i-4)*cell + margin)], fill=c)
			draw.rectangle([(3*cell + margin, (i-5)*cell + margin), (4*cell + margin, (i-4)*cell + margin)], fill=c)
		elif i < 15:
			draw.rectangle([(0*cell + margin, (i-10)*cell + margin), (1*cell + margin, (i-9)*cell + margin)], fill=c)
			draw.rectangle([(4*cell + margin, (i-10)*cell + margin), (5*cell + margin, (i-9)*cell + margin)], fill=c)

	image.save(open(os.path.join(app.config["PFP_FOLDER"], "%d.png" % filename), "w"), "PNG")
	return

from models import db, Config
def is_setup_complete():
	obj = Config.query.filter_by(key="setup_complete").first()
	if obj is None: return False
	return obj.value == True or int(obj.value) == 1

def get_ctf_name():
	name = Config.query.filter_by(key="ctf_name").first()
	if name is None: return "OpenCTF"
	else: return name.value

def get_config(key, default=None):
	config = Config.query.filter_by(key=key).first()
	if config is None:
		return default
	return str(config.value)

def is_ctf_time():
	start = get_config("start_time")
	end = get_config("end_time")

	start = 0 if start is None else int(start)
	end = 0 if end is None else int(end)

	time = get_time_since_epoch()

	if start and end:
		return start < time < end

	if start < time and end == 0:
		return True

	if time < end and start == 0:
		return True

	return False

def get_ssh_keys():
	private_key = get_config("private_key")
	public_key = get_config("public_key")
	if not (private_key and public_key):
		key = RSA.generate(2048)
		private_key = key.exportKey("PEM")
		public_key = key.publickey().exportKey("OpenSSH")
		with app.app_context():
			private = Config("private_key", private_key)
			public = Config("public_key", public_key)
			db.session.add(private)
			db.session.add(public)
			db.session.commit()
			db.session.close()
	return private_key, public_key