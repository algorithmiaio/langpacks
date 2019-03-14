#!/bin/bash

RUN set -e

ENV DEBIAN_FRONTEND noninteractive

RUN echo "deb http://cran.rstudio.com/bin/linux/ubuntu trusty/" >> /etc/apt/sources.list
RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys E084DAB9
RUN apt-get -y update && \
	apt-get install -y r-base=3.4.4-1trusty0 && \
	apt-get install -y r-base-dev=3.4.4-1trusty0 && \
	apt-get install -y libcurl4-gnutls-dev=7.35.0-1ubuntu2.10 # This is needed for RCurl
RUN rm -rf /var/lib/apt/lists/*

RUN echo "options(repos=structure(c(CRAN=\"http://cran.rstudio.com/\")))" > ~/.Rprofile

RUN R --no-save <<EOF
install.packages("https://cran.r-project.org/src/contrib/Archive/pacman/pacman_0.4.1.tar.gz", repos=NULL, type="source")
library(pacman)
install.packages("https://cran.r-project.org/src/contrib/Archive/rjson/rjson_0.2.15.tar.gz")
install.packages("https://cran.r-project.org/src/contrib/base64enc_0.1-3.tar.gz")
EOF

RUN rm ~/.Rprofile

# We want pacman, rjson, and base64enc to be in the docker image. The others can be bind mounted
# in the normal location which we now make sure is empty
RUN echo "R_LIBS_SITE='/usr/local/lib/R/site-library:/usr/lib/R/site-library:/usr/lib/R/library:/usr/local/lib/R/site-library-langserver'" >> /etc/R/Renviron
RUN mv /usr/local/lib/R/site-library /usr/local/lib/R/site-library-langserver

# We want to allow the algo user to install packages system-wide
RUN mkdir /usr/local/lib/R/site-library
RUN chown algo -R /usr/local/lib/R/site-library
