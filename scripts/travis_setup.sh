#!/bin/bash
MYSQL_ROOT_PASSWORD="i_hate_passwords"

echo "Preparing for MySQL installation..."
echo "USE mysql;\nUPDATE user SET password=PASSWORD('$MYSQL_ROOT_PASSWORD') WHERE user='root';\nFLUSH PRIVILEGES;\n" | mysql -u root

echo "Installing dependencies..."
apt-get -y install python
apt-get -y install python-pip libjpeg-dev
apt-get -y install python-dev libmysqlclient-dev
apt-get -y install nginx
apt-get -y install mysql-server memcached
apt-get -y install tmux
apt-get -y install python-nose

echo "Installing pip dependencies..."
pip install -r scripts/requirements.txt

mysql -u root -p"$MYSQL_ROOT_PASSWORD" -e "CREATE DATABASE openctf;"
mysql -u root -p"$MYSQL_ROOT_PASSWORD" -e "CREATE DATABASE openctf_tests;"
