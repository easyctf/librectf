FROM ubuntu:latest

RUN apt-get update && apt-get -y upgrade
RUN apt-get install -y python python-dev python-pip

RUN mkdir /openctf
ADD . /openctf/
WORKDIR /openctf/

RUN pip install -r requirements.txt

CMD ["gunicorn", "--bind", "0.0.0.0:8000", "-w", "4", "OpenCTF:config_app()"]
EXPOSE 8000