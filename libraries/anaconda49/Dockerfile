ENV LANG=C.UTF-8 LC_ALL=C.UTF-8
ENV PATH /opt/conda/bin:$PATH
ENV ANACONDA_ENV=/home/algo/anaconda_environment

RUN apt-get update --fix-missing && \
    apt-get install -y wget bzip2 ca-certificates libglib2.0-0 libxext6 libsm6 libxrender1 liblzma-dev git && \
    apt-get clean

RUN wget --quiet https://github.com/conda-forge/miniforge/releases/download/4.9.0-3/Miniforge3-4.9.0-3-Linux-x86_64.sh -O ~/miniforge.sh && \
    /bin/bash ~/miniforge.sh -b -p /opt/conda && \
    rm ~/miniforge.sh && \
    /opt/conda/bin/conda clean -tipsy && \
    ln -s /opt/conda/etc/profile.d/conda.sh /etc/profile.d/conda.sh && \
    echo ". /opt/conda/etc/profile.d/conda.sh" >> ~/.bashrc && \
    echo "conda activate base" >> ~/.bashrc && \
    find /opt/conda/ -follow -type f -name '*.a' -delete && \
    find /opt/conda/ -follow -type f -name '*.js.map' -delete && \
    /opt/conda/bin/conda install -y --no-update-deps mamba && \
    /opt/conda/bin/conda clean -afy && \
    chown -R $ALGO_UID:$ALGO_UID /opt/conda/

COPY --chown=$ALGO_UID:$ALGO_UID anaconda49/context/condarc /opt/conda/condarc
