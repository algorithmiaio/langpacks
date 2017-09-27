#!/bin/bash

set -e

OPENCV_VERSION=3.2.0
conda remove -y opencv3
LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$PYTHON_LIB_PATH
wget -q -O - https://github.com/opencv/opencv/archive/${OPENCV_VERSION}.tar.gz | tar -xzf -
wget -q -O - https://github.com/opencv/opencv_contrib/archive/${OPENCV_VERSION}.tar.gz | tar -xzf - 

cd opencv-${OPENCV_VERSION}
cmake -DWITH_CUDA=ON -DCMAKE_BUILD_TYPE=RELEASE -DCMAKE_INSTALL_PREFIX=/usr/local/opencv-${OPENCV_VERSION} \
    -DCUDA_ARCH_BIN=$CUDA_ARCH \
    -DPYTHON2_EXECUTABLE=/opt/anaconda2/bin/python2.7 -DBUILD_PNG=1 -DBUILD_JPEG=1 -DWITH_FFMPEG=1 \
    -DPYTHON_INCLUDE_DIR=/opt/anaconda2/include/python2.7 -DWITH_LIBV4L=ON -DWITH_V4L=1 -DWITH_LAPACK=OFF \
    -DBUILD_opencv_python2=ON -DPYTHON2_PACKAGES_PATH=/opt/anaconda2/lib/python2.7/site-packages \
    -DOPENCV_EXTRA_MODULES_PATH=../opencv_contrib-${OPENCV_VERSION}/modules . 

make -j"$(nproc)" install

cd ../
rm -rf opencv-${OPENCV_VERSION}
rm -rf opencv_contrib-${OPENCV_VERSION}

ldconfig

# Give algo user ability to write to updated files
chown -R algo:algo $PYTHON_LIB_PATH/$PYTHON_VERSION/site-packages
