# Lightly modified from https://github.com/docker-library/python/blob/ab8b829cfefdb460ebc17e570332f0479039e918/3.7/stretch/Dockerfile
FROM ubuntu:18.04
RUN DEBIAN_FRONTEND=noninteractive apt-get update && \
  DEBIAN_FRONTEND=noninteractive apt-get -y install gnupg && \
  DEBIAN_FRONTEND=noninteractive apt-get -y install kgpg
RUN apt-get -y update && \
    DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends -y \
        build-essential \
        ca-certificates \
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
        wget && \
    rm -rf /var/lib/apt/lists/*

# MIT license from initial repo
# Copyright (c) 2014 Docker, Inc.
#
# Permission is hereby granted, free of charge, to any person obtaining
# a copy of this software and associated documentation files (the
# "Software"), to deal in the Software without restriction, including
# without limitation the rights to use, copy, modify, merge, publish,
# distribute, sublicense, and/or sell copies of the Software, and to
# permit persons to whom the Software is furnished to do so, subject to
# the following conditions:
#
# The above copyright notice and this permission notice shall be included
# in all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
# EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
# MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
# IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
# CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
# TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
# SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

# ensure local python is preferred over distribution python
ENV PATH /usr/local/bin:$PATH

ENV GPG_KEY 0D96DF4D4110E5C43FBFB17F2D347EA6AA65421D
ENV PYTHON_VERSION 3.7.1

# Setting keyserver per https://github.com/tianon/gosu/issues/17#issuecomment-203074866
RUN set -ex \
    \
    && wget -O python.tar.xz "https://www.python.org/ftp/python/${PYTHON_VERSION%%[a-z]*}/Python-$PYTHON_VERSION.tar.xz" \
    && wget -O python.tar.xz.asc "https://www.python.org/ftp/python/${PYTHON_VERSION%%[a-z]*}/Python-$PYTHON_VERSION.tar.xz.asc" \
    && export GNUPGHOME="$(mktemp -d)" \
    && $(gpg --batch --keyserver hkp://p80.pool.sks-keyservers.net:80 --recv-keys "$GPG_KEY" ||  gpg --batch --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys "$GPG_KEY") \
    && gpg --batch --verify python.tar.xz.asc python.tar.xz \
    && { command -v gpgconf > /dev/null && gpgconf --kill all || :; } \
    && rm -rf "$GNUPGHOME" python.tar.xz.asc \
    && mkdir -p /usr/src/python \
    && tar -xJC /usr/src/python --strip-components=1 -f python.tar.xz \
    && rm python.tar.xz \
    \
    && cd /usr/src/python \
    && gnuArch="$(dpkg-architecture --query DEB_BUILD_GNU_TYPE)" \
    && ./configure \
        --build="$gnuArch" \
        --enable-loadable-sqlite-extensions \
        --enable-shared \
        --with-system-expat \
        --with-system-ffi \
        --without-ensurepip \
    && make -j "$(nproc)" \
    && make install \
    && ldconfig \
    \
    && find /usr/local -depth \
        \( \
            \( -type d -a \( -name test -o -name tests \) \) \
            -o \
            \( -type f -a \( -name '*.pyc' -o -name '*.pyo' \) \) \
        \) -exec rm -rf '{}' + \
    && rm -rf /usr/src/python \
    \
    && python3 --version

# make some useful symlinks that are expected to exist
RUN cd /usr/local/bin \
    && ln -s idle3 idle \
    && ln -s pydoc3 pydoc \
    && ln -s python3 python \
    && ln -s python3-config python-config

# if this is called "PIP_VERSION", pip explodes with "ValueError: invalid truth value '<VERSION>'"
ENV PYTHON_PIP_VERSION 18.1

RUN set -ex; \
    \
    wget -O get-pip.py 'https://bootstrap.pypa.io/get-pip.py'; \
    \
    python get-pip.py \
        --disable-pip-version-check \
        --no-cache-dir \
        "pip==$PYTHON_PIP_VERSION" \
    ; \
    pip --version; \
    \
    find /usr/local -depth \
        \( \
            \( -type d -a \( -name test -o -name tests \) \) \
            -o \
            \( -type f -a \( -name '*.pyc' -o -name '*.pyo' \) \) \
        \) -exec rm -rf '{}' +; \
    rm -f get-pip.py


## Below this line is stuff that is specific to Caffe
## It is based on the instructions here: https://mc.ai/caffe-installation-on-ubuntu-18-04-lts-python-3-6/

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
  caffe-cpu git protobuf-compiler \
  libatlas-base-dev libopenblas-dev \
  libprotobuf-dev libleveldb-dev libsnappy-dev libopencv-dev libhdf5-serial-dev \
  libgflags-dev libgoogle-glog-dev liblmdb-dev libboost-all-dev \
  python3-dev python3-opencv nvidia-cuda-toolkit nvidia-modprobe

RUN git clone https://github.com/BVLC/caffe.git && \
    cd caffe/python && for req in wheel $(cat requirements.txt) pydot; do pip install $req; done && cd ..

RUN cd /caffe && cat Makefile.config.example \
  | python -c "import sys;print(sys.stdin.read().replace('# OPENCV_VERSION := 3', 'OPENCV_VERSION := 3'))" \
  | python -c "import sys;print(sys.stdin.read().replace('/usr/local/cuda', '/usr/lib/nvidia-cuda-toolkit'))" \
  | python -c "import sys;print(sys.stdin.read().replace('PYTHON_INCLUDE := /usr/include/python2.7', ''))" \
  | python -c "import sys;print(sys.stdin.read().replace('		/usr/lib/python2.7/dist-packages/numpy/core/include', ''))" \
  | python -c "import sys;print(sys.stdin.read().replace('# PYTHON_LIBRARIES := boost_python3 python3.5m', 'PYTHON_LIBRARIES := boost_python3 python3.6m'))" \
  | python -c "import sys;print(sys.stdin.read().replace('# PYTHON_INCLUDE := /usr/include/python3.5m', 'PYTHON_INCLUDE := /usr/include/python3.6m'))" \
  | python -c "import sys;print(sys.stdin.read().replace('#                 /usr/lib/python3.5/dist-packages/numpy/core/include', '               /usr/lib/python3.6/dist-packages/numpy/core/include'))" \
  | python -c "import sys;print(sys.stdin.read().replace('/usr/local/include', '/usr/local/include /usr/include/hdf5/serial'))" \
  | python -c "import sys;print(sys.stdin.read().replace('/usr/local/lib /usr/lib', '/usr/local/lib /usr/lib/x86_64-linux-gnu/hdf5/serial'))" \
  | python -c "import sys;print(sys.stdin.read().replace('-gencode arch=compute_20,code=sm_20', '-gencode arch=compute_62,code=sm_62'))" \
  | python -c "import sys;print(sys.stdin.read().replace('-gencode arch=compute_20,code=sm_21', ''))" \
  > Makefile.config && \
  make all -j6 && make pycaffe -j6

ENV PYTHONPATH /caffe/python:$PYTHONPATH
RUN pip install --upgrade pip && pip install matplotlib algorithmia protobuf
