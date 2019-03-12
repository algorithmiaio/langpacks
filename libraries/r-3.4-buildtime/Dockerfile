COPY --chown=0:0 r-3.4-buildtime/context/build /usr/local/bin/algorithmia-build
COPY --chown=0:0 r-3.4-buildtime/context/test /usr/local/bin/algorithmia-test
COPY --chown=0:0 r-3.4-buildtime/context/copy-cran-packages.py /usr/local/bin/algorithmia-copy-cran-packages.py
COPY --chown=0:0 r-3.4-buildtime/context/rip.py /usr/local/bin/algorithmia-rip.py
ENV PATH=$PATH:/home/algo/.local/bin

# Install Python2.7 to support the 2 python scripts included in the R builder
RUN apt-get -y update
RUN apt-get install -y python2.7