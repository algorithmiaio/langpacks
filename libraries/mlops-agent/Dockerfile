RUN mkdir -p /opt/mlops-agent && mkdir -p /tmp/ta  && \
    wget https://james-s-public.s3.amazonaws.com/mlops-agent.tar.gz && \
    tar -xvf mlops-agent.tar.gz -C /opt/mlops-agent && \
    rm -rf /tmp/mlops-agent.tar.gz
RUN pip install /opt/mlops-agent/datarobot_mlops_package-8.1.2/lib/datarobot_mlops-8.1.2-py2.py3-none-any.whl
RUN chmod 777 -R /opt/mlops-agent/datarobot_mlops_package-8.1.2/bin
RUN chmod 777 /opt/mlops-agent/datarobot_mlops_package-8.1.2/conf/*
RUN touch /opt/mlops-agent/datarobot_mlops_package-8.1.2/logs/mlops.agent.log && \
     touch /opt/mlops-agent/datarobot_mlops_package-8.1.2/logs/mlops.agent.out

RUN chmod 777 /opt/mlops-agent/datarobot_mlops_package-8.1.2/logs/*
RUN chmod 777 /tmp/ta