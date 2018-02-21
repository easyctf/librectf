#!/bin/bash

apt-get update && apt-get install -y \
    build-essential \
    git \
    libffi-dev \
    libjpeg-dev \
    libmysqlclient-dev \
    libpng-dev \
    libssl-dev \
    mysql-client \
    mysql-server \
    nginx \
    openssh-client \
    pkg-config \
    python2.7 \
    python3 \
    python3-dev \
    python3-nose \
    python3-pip \
    realpath \
    redis-server \
    systemd \

(cd server; ./cloud-provision.sh)

(cd filestore; ./cloud-provision.sh)
