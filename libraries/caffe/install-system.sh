#!/bin/bash
# Installing system packages that are required by caffe

set -e

apt-get update  && apt-get install -y \
    libboost-all-dev \
    libleveldb-dev \
    liblmdb-dev \
    libopencv-dev \
    libprotobuf-dev \
    libsnappy-dev \
    protobuf-compiler
