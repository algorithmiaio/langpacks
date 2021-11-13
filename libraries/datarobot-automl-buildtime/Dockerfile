# Install CA certificates and update-ca-certificates command
RUN apt-get update && apt-get install -y ca-certificates
RUN mkdir -p /opt/ml/model && chown 2222:2222 /opt/ml/model
COPY --chown=0:0 datarobot-automl-buildtime/context/build /usr/local/bin/algorithmia-build
