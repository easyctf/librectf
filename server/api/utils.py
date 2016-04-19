import datetime
import hashlib
import json
import random
import re
import requests
import string
import traceback
import unicodedata

from PIL import Image, ImageDraw

from flask import current_app as app
from functools import wraps
from werkzeug.security import generate_password_hash, check_password_hash

__check_email_format = lambda email: re.match(".+@.+\..{2,}", email) is not None
__check_ascii = lambda s: all(c in string.printable for c in s)
__check_alphanumeric = lambda s: all(c in string.digits + string.ascii_uppercase + string.ascii_lowercase for c in s)

def hash_password(s):
	return generate_password_hash(s)

def check_password(hashed_password, try_password):
	return check_password_hash(hashed_password, try_password)

def generate_string(length=32, alpha=string.hexdigits):
	return "".join([random.choice(alpha) for x in range(length)])

def unix_time_millis(dt):
	epoch = datetime.datetime.utcfromtimestamp(0)
	return (dt - epoch).total_seconds() * 1000.0

def get_time_since_epoch():
	return unix_time_millis(datetime.datetime.now())

def flat_multi(multidict):
	flat = {}
	for key, values in multidict.items():
		value = values[0] if type(values) == list and len(values) == 1 else values
		flat[key] = value.encode("utf-8")
	return flat

def send_email(recipient, subject, body):
	api_key = app.config["MG_API_KEY"]
	data = {"from": "OpenCTF Administrator <%s>" % (app.config["ADMIN_EMAIL"]),
			"to": recipient,
			"subject": subject,
			"text": body
			}
	auth = ("api", api_key)
	return requests.post("https://api.mailgun.net/v3/%s/messages" % (app.config["MG_HOST"]), auth=auth, data=data)

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

	image.save(open("pfp/%s.png" % filename, "w"), "PNG")
	return