FROM python:3
ENV FLASK_DEBUG=1

RUN apt-get update -y && apt-get install -y --no-install-recommends \
  libmariadb-dev \
  ;
RUN pip install poetry

ENV WAIT_VERSION 2.7.2
ADD https://github.com/ufoscout/docker-compose-wait/releases/download/$WAIT_VERSION/wait /wait
RUN chmod +x /wait

RUN mkdir -p /app
WORKDIR /app

COPY poetry.lock .
COPY pyproject.toml .
RUN poetry install

CMD ["sh", "-c", "/wait && poetry run flask run --host 0.0.0.0"]
