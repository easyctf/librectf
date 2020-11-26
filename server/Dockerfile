FROM python:3.8-alpine3.11

RUN apk add build-base musl-dev libffi-dev mariadb-dev jpeg-dev

COPY Pipfile /
COPY Pipfile.lock /
RUN pip install pipenv
RUN pipenv install --deploy --system

COPY . /app
WORKDIR /app
EXPOSE 8000
ENTRYPOINT ["pipenv", "run", "/app/entrypoint.sh", "runserver"]
