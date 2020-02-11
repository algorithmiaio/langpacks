ENV APEX_GIT_HASH 5b71d3695bf39efcdcda9dff5be2f70314b8f091
RUN git clone https://github.com/NVIDIA/apex /tmp/apex \
	&& cd "/tmp/apex" \
	&& git reset --hard $APEX_GIT_HASH \
	&&  pip install -v --no-cache-dir --global-option="--cpp_ext" --global-option="--cuda_ext" ./ \
	&& rm -rf /tmp/apex \
	&&  pip install torchvision==0.5.0
