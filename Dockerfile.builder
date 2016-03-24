FROM ubuntu:14.04

RUN DEBIAN_FRONTEND=noninteractive apt-get -y update && \
    DEBIAN_FRONTEND=noninteractive apt-get -y install \
    curl zip software-properties-common build-essential && \
    rm -rf /var/lib/apt/lists/* && \
    adduser --disabled-password --gecos "" algo

WORKDIR /tmp/build
CMD ["bin/build"]

