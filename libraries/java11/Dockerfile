# Pull base image.
RUN apt-get update && apt-get install -y --no-install-recommends \
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
		openjdk-11-jdk-headless \
		&& rm -rf /var/lib/apt/lists/*

# basic smoke test
RUN javac --version; \
	java --version


# ENV JAVA_VERSION 11.0.3
# ENV JAVA_BASE_URL https://github.com/AdoptOpenJDK/openjdk11-upstream-binaries/releases/download/jdk-11.0.3%2B7/OpenJDK11U-
# ENV JAVA_URL_VERSION 11.0.3_7
