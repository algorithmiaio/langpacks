#!/bin/bash

set -e

# Add torch to path for this script (will be added globally after full install)
PATH=/opt/torch/install/bin:$PATH

HDF5_COMMIT=f364b442655b0fe21dafe83104f42c3bb7b2a594
git clone https://github.com/deepmind/torch-hdf5.git  /tmp/torch-hdf5 --recursive \
    && cd /tmp/torch-hdf5 \
    && git checkout "$HDF5_COMMIT" \
    && luarocks make hdf5-0-0.rockspec LIBHDF5_LIBDIR="/usr/lib/x86_64-linux-gnu/" \
    && rm -rf /tmp/torch-hdf5 \
    && cd /

# install n2n
N2N_COMMIT=0bb81a1b8f81c5d93835605871d93fb57b75d21e
git clone https://github.com/soumith/net2net.torch.git /tmp/torch-n2n --recursive \
    && cd /tmp/torch-n2n \
    && git checkout "$N2N_COMMIT" \
    && luarocks make net2net-scm-1.rockspec \
    && rm -rf /tmp/torch-n2n \
    && cd /


# Install Moses for utilities
luarocks install moses 1.4.0-1

# Install CSV parser
luarocks install csv 1-1

# Install torch-autograd, rnn and unsup
luarocks install unsup 0.1-0

AUTOGRAD_COMMIT=83225a23bb0332762a6491208c39f5229d5c8cc6
git clone https://github.com/twitter/torch-autograd.git /tmp/luarocks-autograd --recursive \
    && cd /tmp/luarocks-autograd \
    && git checkout "$AUTOGRAD_COMMIT" \
    && luarocks make autograd-scm-1.rockspec \
    && rm -rf /tmp/luarocks-autograd \
    && cd /


RNN_COMMIT=c1b1d664d118685fc7f4258fced0f4dd2619e8bf
git clone https://github.com/Element-Research/rnn.git /tmp/luarocks-rnn --recursive \
    && cd /tmp/luarocks-rnn \
    && git checkout "$RNN_COMMIT" \
    && luarocks make rocks/rnn-scm-1.rockspec \
    && rm -rf /tmp/luarocks-rnn \
    && cd /


# Install various luarocks packages
DP_COMMIT=8f4c3a3b3bcd5e483f11034591b19cd8bc514668
git clone https://github.com/nicholas-leonard/dp.git /tmp/luarocks-dp --recursive \
    && cd /tmp/luarocks-dp \
    && git checkout "$DP_COMMIT" \
    && luarocks make rocks/dp-scm-1.rockspec \
    && rm -rf /tmp/luarocks-dp \
    && cd /


DPNN_COMMIT=ead91a239317dfcc04a0291cfa719f5f3ce7b6ba
git clone https://github.com/Element-Research/dpnn.git /tmp/luarocks-dpnn --recursive \
    && cd /tmp/luarocks-dpnn \
    && git checkout "$DPNN_COMMIT" \
    && luarocks make rocks/dpnn-scm-1.rockspec \
    && rm -rf /tmp/luarocks-dpnn \
    && cd /


# Install Lua POSIX bindings
luarocks install luaposix 33.4.0-1

# Install random number generator which allows multiple RNG instances
luarocks install lrandom

# luarocks install loadcaffe
#Bug mentioned here: https://github.com/szagoruyko/loadcaffe/issues/30
LOADCAFFE_COMMIT=9be65cf6fa08e9333eae3553f68a8082debe9978

git clone https://github.com/szagoruyko/loadcaffe.git
cd loadcaffe && \
  git checkout "$LOADCAFFE_COMMIT" && \
  sed -i 's/-std=c++11/-std=c++0x/g' CMakeLists.txt && \
  luarocks make && \
  cd ..

# Install nn luarocks
NN_COMMIT=a81143f095c2aeb897f3f043699d10d0c4a96375
git clone https://github.com/torch/nn.git /tmp/luarocks-nn --recursive \
    && cd /tmp/luarocks-nn \
    && git checkout "$NN_COMMIT" \
    && luarocks make rocks/nn-scm-1.rockspec \
    && rm -rf /tmp/luarocks-nn
