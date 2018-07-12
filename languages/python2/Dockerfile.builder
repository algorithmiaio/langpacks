FROM ubuntu:16.04

# Set options that should be defined everywhere
ENV JAVA_TOOL_OPTIONS=-Dfile.encoding=UTF8
ENV LANG C.UTF-8

# Algo uid is set so that it is known for build caches, but the user id
# would presumably not be used already on our host (which seems better for security)
RUN adduser --disabled-password --gecos "" --uid 2222 algo

COPY languages/python2/bin/install-buildtools /tmp/python2/install-buildtools
RUN echo "Setting up python2" ; \
    /tmp/python2/install-buildtools && \
    rm -rf /var/lib/apt/lists/*

COPY languages/python2/bin/build /opt/algorithm/bin/build
COPY languages/python2/bin/test /opt/algorithm/bin/test


ENV PATH=/opt/anaconda2/bin:$PATH

USER algo

WORKDIR /opt/algorithm