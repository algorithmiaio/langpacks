RUN mkdir -p /opt/mlops-agent
RUN wget https://james-s-public.s3.amazonaws.com/mlops-agent.tar.gz
RUN tar -xvf mlops-agent.tar.gz -C /opt/mlops-agent
RUN pip install /opt/mlops-agent/datarobot_mlops_package-8.1.2/lib/datarobot_mlops-8.1.2-py2.py3-none-any.whl

#RUN mkdir -p /opt/mlops-agent && \
#    wget https://james-s-public.s3.amazonaws.com/mlops-agent.tar.gz && \
#    tar -xvjf mlops-agent.tar.gz -C /opt/mlops-agent && \
#    rm -rf /tmp/mlops-agent.tar.gz
#RUN pip install /opt/mlops-agent/lib/datarobot_mlops-*.whl && \
#    sh /opt/mlops-agent/bin/start-agent.sh