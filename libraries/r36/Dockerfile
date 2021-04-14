## https://cran.r-project.org/bin/linux/ubuntu/README.html

RUN apt-get update && apt-get install -y ca-certificates apt-transport-https gnupg2 \
  && apt-key adv --keyserver keyserver.ubuntu.com --recv-keys E298A3A825C0D65DFD57CBB651716619E084DAB9

#This means that we'll be using R 3.6 going forwards and this deb is version locked
RUN echo "deb https://cloud.r-project.org/bin/linux/ubuntu xenial-cran35/" >> /etc/apt/sources.list

RUN apt-get update \
  && apt-get install -y --no-install-recommends \
    build-essential \
    software-properties-common \
    r-base-dev \
    libssl-dev \
    libxml2-dev \
    python-dev \
    apt-transport-https \
    ca-certificates \
    git \
    libcurl4-gnutls-dev \
    wget \
    libv8-dev

RUN Rscript -e "install.packages(c('rjson', 'base64enc', 'RCurl', 'pacman'))"

# We want to allow the algo user to install packages system-wide
RUN chown algo -R /usr/local/lib/R/site-library

