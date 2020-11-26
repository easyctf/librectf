import hashlib
import re
import time
from io import BytesIO
from string import hexdigits
from urllib.parse import urljoin, urlparse

import requests
from flask import current_app, redirect, request, url_for
from PIL import Image, ImageDraw, ImageOps

from easyctf.objects import random

VALID_USERNAME = re.compile(r"^[A-Za-z_][A-Za-z\d_]*$")
VALID_PROBLEM_NAME = re.compile(r"^[a-z_][a-z\-\d_]*$")


def generate_string(length=32, alpha=hexdigits):
    characters = [random.choice(alpha) for x in range(length)]
    return "".join(characters)


def generate_short_string():
    return generate_string(length=16)


def send_mail(recipient, subject, body):
    data = {
        "from": current_app.config["ADMIN_EMAIL"],
        "subject": subject,
        "html": body
    }
    data["bcc" if type(recipient) == list else "to"] = recipient
    auth = ("api", current_app.config["MAILGUN_API_KEY"])
    url = "{}/messages".format(current_app.config["MAILGUN_URL"])
    return requests.post(url, auth=auth, data=data)


def filestore(name):
    prefix = current_app.config.get("FILESTORE_STATIC", "/static")
    return prefix + "/" + name


def save_file(file, **params):
    url = current_app.config.get(
        "FILESTORE_SAVE_ENDPOINT", "http://filestore:5001/save")
    return requests.post(url, data=params, files=dict(file=file))


def to_timestamp(date):
    if date is None:
        return ""
    return int(time.mktime(date.timetuple()))


def to_place_str(n):
    # https://codegolf.stackexchange.com/a/4712
    k = n % 10
    return "%d%s" % (n, "tsnrhtdd"[(n / 10 % 10 != 1) * (k < 4) * k::4])


def is_safe_url(target):
    ref_url = urlparse(request.host_url)
    test_url = urlparse(urljoin(request.host_url, target))
    return test_url.scheme in ("http", "https") and \
        ref_url.netloc == test_url.netloc


def get_redirect_target():
    for target in request.values.get("next"), request.referrer:
        if not target:
            continue
        if is_safe_url(target):
            return target


def redirect_back(endpoint, **values):
    target = request.form["next"]
    if not target or not is_safe_url(target):
        target = url_for(endpoint, **values)
    return redirect(target)


def sanitize_avatar(f):
    try:
        im = Image.open(f)
        im2 = ImageOps.fit(im, (512, 512), Image.ANTIALIAS)

        buf = BytesIO()
        im2.save(buf, format="png")
        buf.seek(0)
        return buf
    except:
        return None


def generate_identicon(seed):
    # forgot where i got this code from but if i find it i'll credit it

    seed = seed.strip().lower().encode("utf-8")
    h = hashlib.sha1(seed).hexdigest()
    size = 256
    margin = 0.08
    base_margin = int(size * margin)
    cell = int((size - base_margin * 2.0) / 5)
    margin = int((size - cell * 5.0) / 2)
    image = Image.new("RGB", (size, size))
    draw = ImageDraw.Draw(image)

    def hsl2rgb(h, s, b):
        h *= 6
        s1 = []
        s *= b if b < 0.5 else 1 - b
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
            s1[~~h % 6], s1[(h | 16) % 6], s1[(h | 8) % 6]
        ]

    rgb = hsl2rgb(int(h[-7:], 16) & 0xfffffff, 0.5, 0.7)
    bg = (255, 255, 255)
    fg = (int(rgb[0] * 255), int(rgb[1] * 255), int(rgb[2] * 255))
    draw.rectangle([(0, 0), (size, size)], fill=bg)

    for i in range(15):
        c = bg if int(h[i], 16) % 2 == 1 else fg
        if i < 5:
            draw.rectangle([(2 * cell + margin, i * cell + margin),
                            (3 * cell + margin, (i + 1) * cell + margin)],
                           fill=c)
        elif i < 10:
            draw.rectangle([(1 * cell + margin, (i - 5) * cell + margin),
                            (2 * cell + margin, (i - 4) * cell + margin)],
                           fill=c)
            draw.rectangle([(3 * cell + margin, (i - 5) * cell + margin),
                            (4 * cell + margin, (i - 4) * cell + margin)],
                           fill=c)
        elif i < 15:
            draw.rectangle(
                [(0 * cell + margin, (i - 10) * cell + margin),
                 (1 * cell + margin, (i - 9) * cell + margin)], fill=c)
            draw.rectangle(
                [(4 * cell + margin, (i - 10) * cell + margin),
                 (5 * cell + margin, (i - 9) * cell + margin)], fill=c)

    return image
