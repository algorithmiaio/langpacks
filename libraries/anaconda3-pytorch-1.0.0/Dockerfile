ENV REQFILE=/home/algo/dependency.yml
COPY --chown=0:0 anaconda3-buildtime/context/build /usr/local/bin/dependency-build
COPY --chown=0:0 anaconda3-pytorch-1.0.0/context/requirements.yml $REQFILE
RUN dependency-build $REQFILE && rm $REQFILE
