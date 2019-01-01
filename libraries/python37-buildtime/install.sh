#!/bin/bash

# Install a handful of build dependencies needed for python
set -e

apt-get -y update

# Install things that are required for building python
apt-get install --no-install-recommends -y \
    build-essential \
    checkinstall \
    libbz2-dev \
    libc6-dev \
    libffi-dev \
    libgdbm-dev \
    libncursesw5-dev \
    libreadline-gplv2-dev \
    libsqlite3-dev \
    libssl-dev \
    tk-dev \
    uuid-dev \
    wget

rm -rf /var/lib/apt/lists/*
