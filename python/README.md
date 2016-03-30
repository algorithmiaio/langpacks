# Python LangPack

How to make a new algorithm:
docker run --rm -it -v `pwd`:/tmp/build -e LANGUAGE_VERSION=[python2 | python3] algorithmia/langbuilder-python
    - Make sure to make the dependencies directory before running this

How to run that algorithm:
docker run --rm -it -v /path/to/algorithm.zip:/tmp/algorithm.zip -p 3000:3000 -e LANGUAGE_VERSION=[python2 | python3] algorithmia/langserver-python

And to get output from the algorithm run
curl -s localhost:3000 -X POST -H 'Content-Type: text/plain' -d '<INPUT>'