# Python LangPack

How to make a new algorithm:
docker run --rm -it -v `pwd`/dependencies:/home/algo/.local -v `pwd`:/tmp/build algorithmia/langbuilder-python
    - Make sure to make the dependencies directory before running this

How to run that algorithm:
docker run --rm -it -v /path/to/algorithm.zip:/tmp/algorithm.zip -p 3000:3000 algorithmia/langserver-python
