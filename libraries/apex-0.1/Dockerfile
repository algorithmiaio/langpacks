ENV APEX_GIT_HASH 5b71d3695bf39efcdcda9dff5be2f70314b8f091
RUN git clone https://github.com/NVIDIA/apex /tmp/apex
WORKDIR "/tmp/apex"
RUN git reset --hard $APEX_GIT_HASH
RUN pip install -v --no-cache-dir --global-option="--cpp_ext" --global-option="--cuda_ext" ./
RUN rm -rf /tmp/apex
RUN pip install torchvision==0.5.0
