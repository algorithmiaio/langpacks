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
        openjdk-8-jdk \
        maven && \
        rm -rf /var/lib/apt/lists/*