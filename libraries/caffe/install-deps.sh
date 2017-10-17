#!/bin/bash
# Install libraries that are needed by caffe
set -e

# For caffe installation, see: https://github.com/BVLC/caffe/wiki/Ubuntu-16.04-or-15.10-Installation-Guide
# https://github.com/BVLC/caffe/issues/4333#issuecomment-228874430
ln -s /usr/lib/x86_64-linux-gnu/libhdf5_serial.so /usr/lib/x86_64-linux-gnu/libhdf5.so
ln -s /usr/lib/x86_64-linux-gnu/libhdf5_serial_hl.so /usr/lib/x86_64-linux-gnu/libhdf5_hl.so

# Numpy include path hack - github.com/BVLC/caffe/wiki/Ubuntu-14.04-VirtualBox-VM
if [[ -z $PYTHON_VERSION ]]; then
    echo "PYTHON_VERSION must be specified (e.g. python2.7)"
    exit 1
fi
if [[ -z $PYTHON_LIB_PATH ]]; then
    echo "Python install path must be specified (e.g. /opt/anaconda2/lib)"
    exit 1
fi
ln -s $PYTHON_LIB_PATH/$PYTHON_VESRION /usr/local/include/$PYTHON_VERSION
ln -s $PYTHON_LIB_PATH/$PYTHON_VERSION/site-packages/numpy/core/include/numpy/ /usr/local/include/$PYTHON_VERSION/numpy

# Glog
cd /opt && curl -LO https://storage.googleapis.com/google-code-archive-downloads/v2/code.google.com/google-glog/glog-0.3.3.tar.gz --cacert /etc/ssl/certs/ca-certificates.crt && \
    tar zxvf glog-0.3.3.tar.gz && \
    cd /opt/glog-0.3.3 && \
    ./configure && \
    make -j && \
    make -j install

# Workaround for error loading libglog:
#   error while loading shared libraries: libglog.so.0: cannot open shared object file
# The system already has /usr/local/lib listed in /etc/ld.so.conf.d/libc.conf, so
# running `ldconfig` fixes the problem (which is simpler than using $LD_LIBRARY_PATH)
# TODO: looks like this needs to be run _every_ time a new docker instance is run,
#       so maybe LD_LIBRARY_PATh is a better approach (or add call to ldconfig in ~/.bashrc)
ldconfig

# Gflags
cd /opt && \
    curl -LO https://github.com/schuhschuh/gflags/archive/master.zip --cacert /etc/ssl/certs/ca-certificates.crt && \
    unzip master.zip && \
    cd /opt/gflags-master && \
    mkdir build && \
    cd /opt/gflags-master/build && \
    export CXXFLAGS="-fPIC" && \
    cmake .. && \
    make VERBOSE=1 && \
    make -j && \
    make -j install
