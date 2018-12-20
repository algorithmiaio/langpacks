#!/bin/bash

set -e

apt-get -y update

apt-get install -u -y \
    python3.7 \
    python3-pip

pip3 install --upgrade pip

rm -rf /var/lib/apt/lists/*
