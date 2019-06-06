## https://cran.r-project.org/bin/linux/ubuntu/README.html

RUN apt-get update && apt-get install -y ca-certificates apt-transport-https \
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
		wget

RUN Rscript -e "install.packages('rjson')" \
    && Rscript -e "install.packages('base64enc')" \
    && Rscript -e "install.packages('RCurl')" \
    && Rscript -e "install.packages('pacman')"

# We want pacman, rjson, and base64enc to be in the docker image. The others can be bind mounted
# in the normal location which we now make sure is empty
RUN echo "R_LIBS_SITE='/usr/local/lib/R/site-library:/usr/lib/R/site-library:/usr/lib/R/library:/usr/local/lib/R/site-library-langserver'" >> /etc/R/Renviron
RUN mv /usr/local/lib/R/site-library /usr/local/lib/R/site-library-langserver

# We want to allow the algo user to install packages system-wide
RUN mkdir /usr/local/lib/R/site-library
RUN chown algo -R /usr/local/lib/R/site-library

