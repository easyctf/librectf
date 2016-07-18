FROM docker

RUN apk add --no-cache python python-dev py-pip

RUN mkdir /sandbox
COPY . /sandbox/
WORKDIR /sandbox

RUN which python

CMD ["./server.py"]
EXPOSE 4000