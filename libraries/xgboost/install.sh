#!/bin/bash

set -e

git clone --recursive https://github.com/dmlc/xgboost && \
    cd xgboost && \
    make -j4 && \
    cd python-package; python setup.py install

# Give algo user ability to write to updated files
chown -R algo:algo $PYTHON_LIB_PATH/$PYTHON_VERSION/site-packages
