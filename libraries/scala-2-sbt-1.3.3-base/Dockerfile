ARG SBT_VERSION=1.3.3

RUN apt-get update && \
	apt-get install -y --no-install-recommends \
		bzip2 \
		unzip \
		wget \
     software-properties-common \
            build-essential \
            ca-certificates \
            libssl-dev \
            apt-transport-https \
            wget \
		xz-utils && \
	rm -rf /var/lib/apt/lists/*


RUN curl -L -o sbt-$SBT_VERSION.deb https://dl.bintray.com/sbt/debian/sbt-$SBT_VERSION.deb && \
  dpkg -i sbt-$SBT_VERSION.deb && \
  rm sbt-$SBT_VERSION.deb && \
  apt-get update && \
  apt-get install sbt && \
  sbt sbtVersion