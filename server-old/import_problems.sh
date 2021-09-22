#!/bin/bash

export $(cat /var/easyctf/env | xargs)
python3 manage.py import ~/problems
