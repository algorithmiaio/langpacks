
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
