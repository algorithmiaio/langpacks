RUN mkdir -p /opt/mlops-agent
RUN wget https://james-s-public.s3.amazonaws.com/mlops-agent.tar.gz
RUN tar -xvjf mlops-agent.tar.gz -C /opt/mlops-agent
RUN pip install /opt/mlops-agent/lib/datarobot_mlops-*.whl

#RUN mkdir -p /opt/mlops-agent && \
#    wget https://james-s-public.s3.amazonaws.com/mlops-agent.tar.gz && \
#    tar -xvjf mlops-agent.tar.gz -C /opt/mlops-agent && \
#    rm -rf /tmp/mlops-agent.tar.gz
#RUN pip install /opt/mlops-agent/lib/datarobot_mlops-*.whl && \
#    sh /opt/mlops-agent/bin/start-agent.sh