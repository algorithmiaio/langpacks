#!/bin/bash

set -e

# pycuda is installed from a custom wheel that builds against cuda 8.0
# https://github.com/lebedov/scikit-cuda/issues/185
pip install https://s3.amazonaws.com/algorithmia-wheels/pycuda-2016.1.2-cp27-none-linux_x86_64.whl

# Give algo user ability to write to updated files
find $PYTHON_LIB_PATH/$PYTHON_VERSION/site-packages -user root | xargs chown algo:algo
