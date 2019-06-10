FROM nvidia/cuda:10.0-cudnn7-devel-ubuntu18.04

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

RUN  apt-get update
RUN DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends -y python3.6
RUN DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends -y python3-pip
RUN pip3 install Algorithmia

RUN DEBIAN_FRONTEND=noninteractive apt  install --no-install-recommends -y caffe-cuda
RUN DEBIAN_FRONTEND=noninteractive apt  install --no-install-recommends -y caffe-cpu


RUN echo "alias python=python3.6">>~/.bashrc && \
  echo "alias pip=pip3">>~/.bashrc

#python -c "import caffe;caffe.set_mode_gpu();import Algorithmia"
