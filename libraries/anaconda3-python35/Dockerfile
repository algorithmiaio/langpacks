ARG REQFILE=/home/algo/dependency-python35.yml
COPY --chown=0:0 anaconda3-python35/context/build /usr/local/bin/dependency-build
COPY --chown=0:0 anaconda3-python35/context/requirements.yml $REQFILE
RUN dependency-build $REQFILE
