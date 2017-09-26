#!/bin/bash

apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
        linux-headers-generic \
        module-init-tools

curl -LO http://us.download.nvidia.com/XFree86/Linux-x86_64/367.57/NVIDIA-Linux-x86_64-367.57.run && \
        chmod +x NVIDIA-Linux-x86_64-367.57.run && \
        sh ./NVIDIA-Linux-x86_64-367.57.run -s --no-kernel-module && \
        rm ./NVIDIA-Linux-x86_64-367.57.run

curl -LO https://s3.amazonaws.com/algorithmia-docker/docker-deps/cuda_8.0.44_linux.run && \
    chmod +x cuda_8.0.44_linux.run && \
    sh ./cuda_8.0.44_linux.run --toolkit --silent && \
    rm ./cuda_8.0.44_linux.run && \
    rm -rf /usr/local/cuda-8.0/doc && \
    rm -rf /usr/local/cuda-8.0/samples

curl -LO https://s3.amazonaws.com/algorithmia-docker/docker-deps/cudnn-8.0-linux-x64-v5.1.tgz && \
    tar -xf cudnn-8.0-linux-x64-v5.1.tgz && \
    mv cuda/include/* /usr/local/cuda/include && \
    mv cuda/lib64/* /usr/local/cuda/lib64 && \
    rm cudnn-8.0-linux-x64-v5.1.tgz

ldconfig /usr/local/cuda/lib64
