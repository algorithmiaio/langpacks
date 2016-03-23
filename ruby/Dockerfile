FROM algorithmia/langserver

ADD bin/install-deps /tmp/
RUN /tmp/install-deps && rm -rf /var/lib/apt/lists/*

ADD bin /algorithmia/bin
