FROM mysql:latest

COPY conf.d/* /etc/mysql/conf.d/
RUN chmod 644 /etc/mysql/conf.d/*

CMD ["mysqld"]