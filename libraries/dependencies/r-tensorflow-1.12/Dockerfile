RUN echo ". /opt/conda/etc/profile.d/conda.sh" >> /etc/environment && \
    echo "conda activate base" >> /etc/environment

ENV PATH /opt/conda/bin:$PATH

RUN conda install python=3.6 && \
/opt/conda/bin/pip install tensorflow==1.12 && \
/opt/conda/bin/pip install keras==2.1.4 && \
/opt/conda/bin/pip install h5py==2.9.0

ENV RETICULATE_PYTHON /opt/conda/bin/python
