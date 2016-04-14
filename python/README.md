# Python LangPack

This LangPack provide support for building and running Python algorithms on the Algorithmia platform.

## Building an algorithm
```
docker run --rm -it -v `pwd`:/tmp/build -e LANGUAGE_VERSION=[python2 | python3] algorithmia/langbuilder-python
```

## Running an algorithm:
```
docker run --rm -it -v /path/to/algorithm.zip:/tmp/algorithm.zip -p 9999:9999 -e LANGUAGE_VERSION=[python2 | python3] algorithmia/langserver-python
```

## Calling an algorithm
```
curl -s localhost:9999 -X POST -H 'Content-Type: text/plain' -d '<INPUT>'
```