ENV REQFILE=/home/algo/dependency-solaris-0.1.2.yml
COPY --chown=0:0 anaconda3-solaris-0.1.2/context/build /tmp/dependency-solaris-build
COPY --chown=0:0 anaconda3-solaris-0.1.2/context/requirements.yml $REQFILE
RUN sh /tmp/dependency-solaris-build $REQFILE