COPY --chown=0:0 solaris-0.1.2/context/solaris-build /usr/local/bin/solaris-build
RUN git clone https://github.com/CosmiQ/solaris.git /tmp/solaris && \
    chown -R algo:algo /tmp/solaris && \
    cd /tmp/solaris && \
    git checkout v0.1.2 && \
    solaris-build && \
    rm -rf /tmp/solaris*
