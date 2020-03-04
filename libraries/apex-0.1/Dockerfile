ENV APEX_GIT_HASH 5b71d3695bf39efcdcda9dff5be2f70314b8f091

RUN apt-get clean && apt-get update && apt-get install -y --no-install-recommends \
		cuda-nvml-dev-$CUDA_PKG_VERSION \
		cuda-command-line-tools-$CUDA_PKG_VERSION \
		cuda-libraries-dev-$CUDA_PKG_VERSION \
		cuda-minimal-build-$CUDA_PKG_VERSION \
		libnccl-dev=$NCCL_VERSION-1+cuda10.1 \
		libcublas-dev=10.2.1.243-1 \
		libcudnn7=$CUDNN_VERSION-1+cuda10.1 \
		libcudnn7-dev=$CUDNN_VERSION-1+cuda10.1
RUN pip install torchvision==0.5.0
RUN git clone https://github.com/NVIDIA/apex /tmp/apex \
	&& cd "/tmp/apex" \
	&& git reset --hard $APEX_GIT_HASH \
	&& pip install -v --no-cache-dir --global-option="--cpp_ext" --global-option="--cuda_ext" ./ \
	&& rm -rf /tmp/apex \
	&& rm -rf /var/lib/apt/lists/*