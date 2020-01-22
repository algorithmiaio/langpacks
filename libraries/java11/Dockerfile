# Pull base image.

RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		bzip2 \
		unzip \
		wget \
		gnupg \
		gnupg2 \
     software-properties-common \
            build-essential \
            ca-certificates \
            libssl-dev \
            apt-transport-https \
            wget \
            maven \
		xz-utils \
		fontconfig libfreetype6 \
	; \
	rm -rf /var/lib/apt/lists/*


ENV LANG C.UTF-8

ENV JAVA_HOME /usr/local/openjdk-11
ENV PATH $JAVA_HOME/bin:$PATH

# backwards compatibility shim
RUN { echo '#/bin/sh'; echo 'echo "$JAVA_HOME"'; } > /usr/local/bin/docker-java-home && chmod +x /usr/local/bin/docker-java-home && [ "$JAVA_HOME" = "$(docker-java-home)" ]

# https://adoptopenjdk.net/upstream.html
ENV JAVA_VERSION 11.0.3
ENV JAVA_BASE_URL https://github.com/AdoptOpenJDK/openjdk11-upstream-binaries/releases/download/jdk-11.0.3%2B7/OpenJDK11U-
ENV JAVA_URL_VERSION 11.0.3_7
# https://github.com/docker-library/openjdk/issues/320#issuecomment-494050246

RUN set -eu && \
for key in CA5F11C6CE22644D42C6AC4492EF8D39DC13168F EAC843EBD3EFDB98CC772FADA5CD6035332FA671 0x332FA671 ; \
        do \
            gpg --keyserver ha.pool.sks-keyservers.net --recv-keys "$key" || \
            gpg --keyserver pgp.mit.edu --recv-keys "$key" || \
            gpg --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys "$key" || \
            gpg --keyserver keyserver.pgp.com --recv-keys "$key" ; \
        done; \
	dpkgArch="$(dpkg --print-architecture)"; \
	case "$dpkgArch" in \
		amd64) upstreamArch='x64' ;; \
		arm64) upstreamArch='aarch64' ;; \
		*) echo >&2 "error: unsupported architecture: $dpkgArch" ;; \
	esac; \
	\
	wget -O openjdk.tgz.asc "${JAVA_BASE_URL}${upstreamArch}_linux_${JAVA_URL_VERSION}.tar.gz.sign"; \
	wget -O openjdk.tgz "${JAVA_BASE_URL}${upstreamArch}_linux_${JAVA_URL_VERSION}.tar.gz" --progress=dot:giga; \
	\
	mkdir -p "$JAVA_HOME"; \
	tar --extract --file openjdk.tgz --directory "$JAVA_HOME" --strip-components 1; \
	rm openjdk.tgz*; \
	\
# TODO strip "demo" and "man" folders?
	\
# basic smoke test
	javac --version; \
	java --version


ENV JAVA_VERSION 11.0.3
ENV JAVA_BASE_URL https://github.com/AdoptOpenJDK/openjdk11-upstream-binaries/releases/download/jdk-11.0.3%2B7/OpenJDK11U-
ENV JAVA_URL_VERSION 11.0.3_7
