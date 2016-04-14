# Ruby LangPack

This LangPack provide support for building and running Ruby algorithms on the Algorithmia platform.

## Building an algorithm
```
docker run --rm -it -v `pwd`:/tmp/build algorithmia/langbuilder-ruby
```

## Running an algorithm:
```
docker run --rm -it -v /path/to/algorithm.zip:/tmp/algorithm.zip -p 9999:9999 algorithmia/langserver-ruby
```

## Calling an algorithm
```
curl -s localhost:9999 -X POST -H 'Content-Type: text/plain' -d '<INPUT>'
```