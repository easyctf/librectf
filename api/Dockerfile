FROM ubuntu:latest

RUN apt-get update
RUN apt-get -y install python python-pip libjpeg-dev python-dev libmysqlclient-dev git memcached tmux python-nose mysql-client libffi-dev libssl-dev
RUN pip install Flask cryptography==1.2.3 mysql-python Flask-SQLAlchemy SQLAlchemy gunicorn requests voluptuous Pillow markdown2 onetimepass pyqrcode python-memcached nose coverage gitpython pyyaml pycrypto paramiko

RUN mkdir /openctf
COPY . /openctf/
WORKDIR /openctf

RUN pip install -r requirements.txt

CMD ["gunicorn", "--bind", "0.0.0.0:8000", "-w", "4", "app:app"]
EXPOSE 8000