#!/bin/bash

# change the following values if you changed configuration
MYSQL_USER=root
MYSQL_PASS=password
MYSQL_HOST=localhost

mkdir -p backups
(docker exec -it openctf_db_1 mysqldump -u"$MYSQL_USER" -p"$MYSQL_PASS" -h"$MYSQL_HOST" openctf) > backups/$(date +%Y.%m.%d.%H.%M)

# [Optional] You can create a cron job for this script to run it on an interval.