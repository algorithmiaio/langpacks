FROM nvidia/cuda:10.0-cudnn7-devel-ubuntu16.04

RUN apt-get -y update && \
    DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends -y gnupg kgpg

#RUN apt-get -y update && \
#    DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends -y apt-get install  libboost-dev

RUN apt-get -y update && \
    DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends -y \
        build-essential \
        ca-certificates \
        checkinstall \
        libbz2-dev \
        git \
        cmake \
        make \
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

ENV PATH /usr/local/bin:$PATH
ENV GPG_KEY 0D96DF4D4110E5C43FBFB17F2D347EA6AA65421D
ENV PYTHON_VERSION 3.6.8
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


#
#
#


RUN apt-get update && apt-get install -y --no-install-recommends \
        build-essential \
        git \
        wget \
        libatlas-base-dev \
        libboost-all-dev \
        libgflags-dev \
        libgoogle-glog-dev \
        libhdf5-serial-dev \
        libleveldb-dev \
        liblmdb-dev \
        libopencv-dev \
        libprotobuf-dev \
        libsnappy-dev \
        protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*

# this part based on https://askubuntu.com/questions/944035/installing-libboost-python-dev-for-python3-without-installing-python2-7
RUN cd /usr/src && \
 wget --no-verbose https://dl.bintray.com/boostorg/release/1.65.1/source/boost_1_65_1.tar.gz && \
 tar xzf boost_1_65_1.tar.gz && \
 cd boost_1_65_1 && \
 ln -s /usr/local/include/python3.6m /usr/local/include/python3.6 && \
 ./bootstrap.sh --with-python=$(which python3) && \
 ./b2 install && \
 ldconfig && \
 cd / && rm -rf /usr/src/*

RUN git clone https://github.com/BVLC/caffe.git && \
   cd caffe/python && for req in wheel $(cat requirements.txt) pydot; do pip install $req; done && cd ..

RUN cd /caffe && cat Makefile.config.example \
| python -c "import sys;print(sys.stdin.read().replace('# CPU_ONLY := 1', 'CPU_ONLY := 1'))" \
| python -c "import sys;print(sys.stdin.read().replace('-gencode arch=compute_20,code=sm_20', ''))" \
| python -c "import sys;print(sys.stdin.read().replace('-gencode arch=compute_20,code=sm_21', ''))" \
| python -c "import sys;print(sys.stdin.read().replace('PYTHON_INCLUDE := /usr/include/python2.7', ''))" \
| python -c "import sys;print(sys.stdin.read().replace('		/usr/lib/python2.7/dist-packages/numpy/core/include', ''))" \
| python -c "import sys;print(sys.stdin.read().replace('# PYTHON_LIBRARIES := boost_python3 python3.5m', 'PYTHON_LIBRARIES := boost_python3 python3.6m'))" \
| python -c "import sys;print(sys.stdin.read().replace('# PYTHON_INCLUDE := /usr/include/python3.5m', 'PYTHON_INCLUDE := /usr/local/include/python3.6m'))" \
| python -c "import sys;print(sys.stdin.read().replace('#                 /usr/lib/python3.5/dist-packages/numpy/core/include', ' /usr/local/lib/python3.6/site-packages/numpy/core/include'))" \
| python -c "import sys;print(sys.stdin.read().replace('PYTHON_INCLUDE) /usr/local/include', 'PYTHON_INCLUDE) /usr/local/include /usr/include/hdf5/serial'))" \
| python -c "import sys;print(sys.stdin.read().replace('/usr/local/lib /usr/lib', '/usr/local/lib /usr/lib /usr/lib/x86_64-linux-gnu /usr/lib/x86_64-linux-gnu/hdf5/serial'))" \
> Makefile.config && \
make all -j$(nproc) && make pycaffe -j$(nproc)

ENV CAFFE_ROOT /caffe
ENV PYCAFFE_ROOT $CAFFE_ROOT/python
ENV PYTHONPATH $PYCAFFE_ROOT:$PYTHONPATH
