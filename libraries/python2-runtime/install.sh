#!/bin/bash

set -e

apt-get -y update

apt-get install -u -y \
    python2.7 \
    python-pip

pip install --upgrade pip

rm -rf /var/lib/apt/lists/*
