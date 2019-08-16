ARG REQFILE=/home/algo/dependency-python27.yml
COPY --chown=0:0 anaconda3-python27/context/build /usr/local/bin/dependency-build
COPY --chown=0:0 anaconda3-python27/context/requirements.yml $REQFILE
RUN dependency-build $REQFILE
