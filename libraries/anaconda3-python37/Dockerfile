ENV REQFILE=/home/algo/dependency-python37.yml
COPY --chown=0:0 anaconda3-python37/context/build /usr/local/bin/dependency-build
COPY --chown=0:0 anaconda3-python37/context/requirements.yml $REQFILE
RUN dependency-build $REQFILE
