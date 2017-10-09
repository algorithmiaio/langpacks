#!/bin/bash

set -e

CAFFE_COMMIT=b86b0aea60ad774e131707e00b4150a19dd3d2d5
# Build Caffe core
echo "Installing Caffe..."
cd /opt && git clone https://github.com/BVLC/caffe.git
# See: https://github.com/BVLC/caffe/issues/2347 for the weird find command
cd /opt/caffe && \
    git checkout "$CAFFE_COMMIT" &&\
    cp Makefile.config.example Makefile.config && \
    echo "CPU_ONLY := 0" >> Makefile.config && \
    echo "DEBUG:= 1" >> Makefile.config && \
    find . -type f -exec sed -i -e 's^"hdf5.h"^"hdf5/serial/hdf5.h"^g' -e 's^"hdf5_hl.h"^"hdf5/serial/hdf5_hl.h"^g' '{}' \; && \
    make all -j$(nproc)


# Add ld-so.conf so it can find libcaffe.so
mv /opt/algorithmia/setup/caffe/caffe-ld-so.conf /etc/ld.so.conf.d/

#This is necessary now due to ssl cert failures: http://stackoverflow.com/questions/29134512/insecureplatformwarning-a-true-sslcontext-object-is-not-available-this-prevent
pip install --upgrade requests[security]

# Install python deps
cd /opt/caffe && \
    cat python/requirements.txt | xargs -L 1 pip install

# Build Caffe python bindings
cd /opt/caffe && make pycaffe

# Make all
cd /opt/caffe && make all

# Give algo user ability to write to updated files
find $PYTHON_LIB_PATH/$PYTHON_VERSION/site-packages -user root | xargs chown algo:algo
