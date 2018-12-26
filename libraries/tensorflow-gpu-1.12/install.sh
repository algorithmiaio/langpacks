#!/bin/bash

set -e

# Install the requirements defined in the requirements.txt file in the template
pip install -r ../../templates/tensorflow-gpu-1.12/requirements.txt

# Give algo user ability to write to updated keras/tensorflow files
find $PYTHON_LIB_PATH/$PYTHON_VERSION/site-packages -user root | xargs chown algo:algo