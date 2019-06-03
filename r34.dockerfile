# Kaniko does not support copying from an absolute image name, see: https://github.com/GoogleContainerTools/kaniko/issues/297
FROM algorithmiahq/langserver:ce3f89098fddfaf4db8639a97ce3c0317abbd971 AS langserver
FROM ubuntu:16.04

# Set options that should be defined everywhere
ENV JAVA_TOOL_OPTIONS=-Dfile.encoding=UTF8
ENV LANG C.UTF-8
LABEL langpacks_version=
LABEL langserver_image=algorithmiahq/langserver:ce3f89098fddfaf4db8639a97ce3c0317abbd971

# Algo uid is set so that it is known for build caches, but the user id
# would presumably not be used already on our host (which seems better for security)
ENV ALGO_UID=2222
RUN adduser --disabled-password --gecos "" --uid $ALGO_UID algo
COPY --from=langserver /bin/init-langserver /bin/init-langserver
COPY --from=langserver /langserver/target/release/langserver /bin/langserver




## https://cran.r-project.org/bin/linux/ubuntu/README.html



RUN apt-get update && apt-get install -y ca-certificates apt-transport-https \

&& apt-key adv --keyserver keyserver.ubuntu.com --recv-keys E298A3A825C0D65DFD57CBB651716619E084DAB9



#This means that we'll be using R 3.6 going forwards and this deb is version locked

RUN echo "deb https://cloud.r-project.org/bin/linux/ubuntu xenial-cran35/" >> /etc/apt/sources.list



RUN apt-get update \

	&& apt-get install -y --no-install-recommends \

		software-properties-common \

		r-base-dev \

		git \

		curl \

		wget



RUN Rscript -e "install.packages('rjson')" \

    && Rscript -e "install.packages('base64enc')" \

    && Rscript -e "install.packages('RCurl')"



# We want pacman, rjson, and base64enc to be in the docker image. The others can be bind mounted

# in the normal location which we now make sure is empty

RUN echo "R_LIBS_SITE='/usr/local/lib/R/site-library:/usr/lib/R/site-library:/usr/lib/R/library:/usr/local/lib/R/site-library-langserver'" >> /etc/R/Renviron

RUN mv /usr/local/lib/R/site-library /usr/local/lib/R/site-library-langserver



# We want to allow the algo user to install packages system-wide

RUN mkdir /usr/local/lib/R/site-library

RUN chown algo -R /usr/local/lib/R/site-library







USER $ALGO_UID

WORKDIR /opt/algorithm
ENTRYPOINT /bin/init-langserver