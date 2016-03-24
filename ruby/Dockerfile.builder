FROM algorithmia/langbuilder

ADD bin/setup /tmp/
RUN /tmp/setup && \
    rm -rf /var/lib/apt/lists/*

USER algo
