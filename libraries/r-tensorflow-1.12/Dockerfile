# install miniconda, and set the appropriate path variables.
RUN wget --quiet https://repo.anaconda.com/miniconda/Miniconda3-4.6.14-Linux-x86_64.sh -O ~/miniconda.sh && \
    /bin/bash ~/miniconda.sh -b -p /opt/conda && \
    rm ~/miniconda.sh && \
    /opt/conda/bin/conda clean -tipsy && \
    ln -s /opt/conda/etc/profile.d/conda.sh /etc/profile.d/conda.sh && \
    echo ". /opt/conda/etc/profile.d/conda.sh" >> /etc/environment && \
    echo "conda activate base" >> /etc/environment
ENV PATH /opt/conda/bin:$PATH

RUN conda install python=3.6 && \
/opt/conda/bin/pip install tensorflow==1.12 && \
/opt/conda/bin/pip install keras==2.1.4 && \
/opt/conda/bin/pip install h5py==2.9.0

ENV RETICULATE_PYTHON /opt/conda/bin/python
