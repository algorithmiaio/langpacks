COPY --chown=0:0 anaconda3-buildtime/context/build /usr/local/bin/algorithmia-build
RUN conda create --prefix /tmp/conda_environment && \
    chmod --recursive ugo+rw /tmp/conda_environment