#!/bin/bash

set -e
# Add torch to path for this script (will be added globally after full install)
PATH=/opt/torch/install/bin:$PATH

CUNNX_COMMIT=80e55ade1dc47b8010b238a67a81a0029ee631d8
git clone https://github.com/nicholas-leonard/cunnx.git /tmp/luarocks-cunnx --recursive \
    && cd /tmp/luarocks-cunnx \
    && git checkout "$CUNNX_COMMIT" \
    && luarocks make rocks/cunnx-scm-1.rockspec \
    && rm -rf /tmp/luarocks-cunnx \
    && cd /


CCN2_COMMIT=ac3d21328b329c1d9bd769d97d0f722c6798b013
git clone https://github.com/soumith/cuda-convnet2.torch.git /tmp/luarocks-ccn2 --recursive \
    && cd /tmp/luarocks-ccn2 \
    && git checkout "$CCN2_COMMIT" \
    && luarocks make ccn2-scm-1.rockspec \
    && rm -rf /tmp/luarocks-ccn2 \
    && cd /

