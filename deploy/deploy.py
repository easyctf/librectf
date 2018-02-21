import paramiko
import yaml
import os
import sys


def read_config():
    with open(os.path.join(os.path.dirname(__file__), "config.yml")) as f:
        data = yaml.load(f)
    return data


def get_client():
    client = paramiko.SSHClient()
    client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
    return client


def update_web_server(client, host, pkey):
    client.connect(host, username="root", pkey=pkey)
    (sin, sout, serr) = client.exec_command("/bin/bash -c 'cd /root/easyctf-platform && git reset --hard && git pull origin master && systemctl restart easyctf'")
    print(sout.read(), serr.read())
    client.close()


def reimport_problems(client, host, pkey):
    client.connect(host, username="root", pkey=pkey)
    (sin, sout, serr) = client.exec_command("/bin/bash -c 'cd /root/problems && git reset --hard && git pull origin master && cd /root/easyctf-platform/server && dotenv /var/easyctf/env python3 manage.py import /root/problems'")
    print(sout.read(), serr.read())
    client.close()


def update_judge(client, host, pkey):
    client.connect(host, username="root", pkey=pkey)
    (sin, sout, serr) = client.exec_command("/bin/bash -c 'cd /root/easyctf-platform && git reset --hard && git pull origin master && systemctl restart judge'")
    print(sout.read(), serr.read())
    client.close()


if __name__ == "__main__":
    service = None
    if len(sys.argv) > 1:
        service = sys.argv[1]
    config = read_config()
    key_path = os.path.expanduser(config.get("private_key"))
    pkey = paramiko.RSAKey.from_private_key_file(key_path)
    client = get_client()
    if not service or service == "web":
        for host in config.get("web"):
            update_web_server(client, host, pkey)
            reimport_problems(client, host, pkey)
    if not service or service == "judge":
        for host in config.get("judge"):
            update_judge(client, host, pkey)
