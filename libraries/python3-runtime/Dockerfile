COPY --chown=0:0 python3-runtime/context/pipe /usr/local/bin/algorithmia-pipe
COPY --chown=0:0 python3-runtime/context/pipe-legacy-fallback /usr/local/bin/pipe-legacy-fallback
ENV PATH=$PATH:/home/algo/.local/bin
RUN pip install six