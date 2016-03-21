FROM ubuntu:14.04

RUN DEBIAN_FRONTEND=noninteractive apt-get -y update && \
    DEBIAN_FRONTEND=noninteractive apt-get -y install \
    curl zip software-properties-common && \
    rm -rf /var/lib/apt/lists/*


WORKDIR /algorithmia
EXPOSE 3000
RUN mkfifo /tmp/algoout
ADD init-langserver /bin/
ADD target/release/langserver /bin/langserver
CMD ["/bin/init-langserver"]
