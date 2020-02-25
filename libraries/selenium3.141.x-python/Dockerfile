
# PhantomJS for Selenium
RUN wget https://bitbucket.org/ariya/phantomjs/downloads/phantomjs-2.1.1-linux-x86_64.tar.bz2 && \
  tar -xvjf phantomjs-2.1.1-linux-x86_64.tar.bz2 -C /usr/local/share/ && \
  ln -s /usr/local/share/phantomjs-2.1.1-linux-x86_64/bin/phantomjs /usr/local/bin/

# Chrome & Firefox for Selenium
RUN apt-get update && \
  apt-get install unzip && \
  wget https://chromedriver.storage.googleapis.com/2.41/chromedriver_linux64.zip && \
  unzip chromedriver_linux64.zip && \
  cp chromedriver /usr/local/share/ && \
  ln -s /usr/local/share/chromedriver /usr/local/bin/ && \
  wget https://github.com/mozilla/geckodriver/releases/download/v0.26.0/geckodriver-v0.26.0-linux64.tar.gz && \
  tar xf geckodriver-v0.26.0-linux64.tar.gz && \
  cp geckodriver /usr/local/share/ && \
  ln -s /usr/local/share/geckodriver /usr/local/bin && \
  DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends -y  chromium-browser firefox && \
  rm -rf /var/lib/apt/lists/*

RUN pip install "selenium>=3.141.0,<3.142.0"
