FROM python:3.8-alpine3.11

COPY requirements.txt /
RUN pip install -r requirements.txt

COPY . /filestore
WORKDIR /filestore
EXPOSE 80/tcp
ENTRYPOINT ["python", "-m", "flask", "run", "--host", "0.0.0.0", "--port", "80"]
