RUN git clone https://github.com/CosmiQ/solaris.git && \
    cd solaris && \
    git checkout v0.1.2 && \
    conda env create -n solaris --file=environment.yml -q --force && \
    conda activate solaris && \
    pip install . && \
    rm -rf /tmp/solaris*