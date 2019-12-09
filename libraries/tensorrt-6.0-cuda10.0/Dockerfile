# This is a unique packageset in which it requires both a specific version of CUDA, and a version of Ubuntu as well.
COPY --chown=0:0 tensorrt-6.0-cuda10.0/context/install /tmp/install
RUN wget -O - https://james-s-public.s3.amazonaws.com/TensorRT-6.0.1.5.Ubuntu-16.04.x86_64-gnu.cuda-10.0.cudnn7.6.tar.gz | (cd /tmp && tar zxf -)
RUN sh /tmp/install 6.0.1.5
