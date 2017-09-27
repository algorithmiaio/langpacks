#!/bin/bash

set -e

# Install OpenCV 3.2.0 with Python2 and Python3 support
# To prevent libpcre.so error for importing cv2 in (anaconda) python3
RUN cp -r /root/anaconda3/pkgs/pcre-8.39-1* /opt/anaconda3/pkgs/
ENV LD_LIBRARY_PATH=/opt/anaconda3/pkgs/pcre-8.39-1/lib:$LD_LIBRARY_PATH

OPENCV_VERSION=3.2.0
# Install version 3.2.0 of OpenCV (with ffmpeg support) to Anaconda3
conda remove -y opencv3
LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$PYTHON_INSTALL/lib
wget -q -O - https://github.com/opencv/opencv/archive/${OPENCV_VERSION}.tar.gz | tar -xzf -
wget -q -O - https://github.com/opencv/opencv_contrib/archive/${OPENCV_VERSION}.tar.gz | tar -xzf - 

cd opencv-${OPENCV_VERSION}
cmake -DWITH_CUDA=ON -DCMAKE_BUILD_TYPE=RELEASE -DCMAKE_INSTALL_PREFIX=/usr/local/opencv-${OPENCV_VERSION} \
    -DPYTHON3_EXECUTABLE=/opt/anaconda3/bin/python3.5 -DBUILD_PNG=1 -DBUILD_JPEG=1 -DWITH_FFMPEG=1 \
    -DPYTHON_INCLUDE_DIR=/opt/anaconda3/include/python3.5m -DWITH_LIBV4L=ON -DWITH_V4L=1 -DWITH_LAPACK=OFF \
    -DBUILD_opencv_python2=OFF -DBUILD_opencv_python3=ON -DPYTHON3_PACKAGES_PATH=/opt/anaconda3/lib/python3.5/site-packages \
    -DOPENCV_EXTRA_MODULES_PATH=../opencv_contrib-${OPENCV_VERSION}/modules . && \

make -j"$(nproc)" install

cd ../
rm -rf opencv-${OPENCV_VERSION}
rm -rf opencv_contrib-${OPENCV_VERSION}

ldconfig

