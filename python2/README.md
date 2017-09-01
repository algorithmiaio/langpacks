# Python LangPack

This LangPack provide support for building and running Python2 algorithms on the Algorithmia platform.

## Building an algorithm
```
docker run --rm -it -v `pwd`:/tmp/build  algorithmia/langbuilder-python2

If you want to have a shared cache add this to the command `-v <cache-dir>:/home/algo/.cache`
```

## Running an algorithm:
```
docker run --rm -it -v /path/to/algorithm.zip:/tmp/algorithm.zip -p 9999:9999 algorithmia/langserver-python2
```

## Calling an algorithm
```
curl -s localhost:9999 -X POST -H 'Content-Type: text/plain' -d '<INPUT>'
```

