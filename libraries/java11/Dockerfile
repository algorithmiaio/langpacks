# Pull base image.

RUN apt-get -y update && \
    apt-get install --no-install-recommends -y \
        software-properties-common \
        build-essential \
        ca-certificates \
        libssl-dev \
        apt-transport-https \
        wget \
        default-jdk \
        maven && \
	add-apt-repository ppa:openjdk-r/ppa && \
	apt-get update && \
	apt-get install -y \
	openjdk-11-jdk && \
	rm -rf /var/lib/apt/lists/*
