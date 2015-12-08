FROM ubuntu:latest

RUN apt-key adv --keyserver hkp://keyserver.ubuntu.com:80 --recv 7F0CEB10
RUN echo "deb http://repo.mongodb.org/apt/ubuntu trusty/mongodb-org/3.0 multiverse" | tee /etc/apt/sources.list.d/mongodb-org-3.0.list

RUN apt-get update && apt-get -y upgrade
RUN apt-get install -y mongodb-org libffi-dev python python-dev python-pip

RUN mkdir /openctf
ADD . /openctf/
WORKDIR /openctf/

RUN pip install -r requirements.txt

CMD ["gunicorn", "--bind", "0.0.0.0:8000", "-w", "4", "OpenCTF:config_app()"]
EXPOSE 8000