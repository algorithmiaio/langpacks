COPY --chown=0:0 anaconda3-solaris-0.1.2/context/solaris-build /usr/local/bin/solaris-build
RUN git clone --single-branch --branch v0.1.2 https://github.com/CosmiQ/solaris.git /tmp/solaris && \
    /usr/local/bin/solaris-build /tmp/solaris
