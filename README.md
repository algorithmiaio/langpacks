# LangPacks

This is work-in-progress tracking the [LangPack spec proposal](https://docs.google.com/a/algorithmia.io/document/d/1vd80VKXX5kPIYIHpaXV-oD15aw2CyoM6vXWQLTH9MfI/edit?usp=sharing)

*LangPack*: A language specific package the encompasses steps to setup, build, and run language-specific algorithms.

*LangServer*: A server that serve a LangPack's `bin/pipe` runner in a way that emulates a light-weight version of the Algorithmia API.

## Building LangServer(s)

Disclaimer: The intent was to prototype langserver in rust (because I knew it better), but finally write it in go (lower barrier to entry), but it turned into an official project before the rewrite happened. So, for now: start by installing [latest stable rust](https://www.rust-lang.org/downloads.html), and then:

```
bin/build langserver # just builds the base langserver images (default)
bin/build <lang>     # builds language-specific image (and deps)
bin/build all        # builds all images for all langpacks
```

Note: the initial plan is to NOT use these images, but they are helpful for implementing and testing langpacks locally, as well as provide some "code documentation" for how setup/build/pipe/langserver all fit together.

## Building an algorithm

Bind mount an algorithm working directory to `/tmp/build` and start the langbuilder-<lang> image. It should create an algorithm.zip that can be served by the init-langserver script (containing `bin/pipe`, the algorithm, and any dependencies):

```
docker run --rm -it -v `pwd`:/tmp/build algorithmia/langbuilder-<lang>
```

Note, unless using Docker user namespacing, don't be shocked if bind-mount writing results in permission errors.

## Running LangServer

The `init-langserver` script provides 2 ways to run an algorithm:

#### Bind mount algorithm.zip to /tmp/algorithm.zip
Note: Make sure you use the absolute path to the algorithm.zip.
```
docker run --rm -it -v /path/to/algorithm.zip:/tmp/algorithm.zip -p 9999:9999 algorithmia/langserver-<lang>
```

#### Bind mount algorithm directory to /tmp/algorithm
```
docker run --rm -it -v `pwd`:/tmp/algorithm -p 9999:9999 algorithmia/langserver-<lang>
```

## Contributing

Bonus 🌮🌮tacos🌮🌮 for you if you write a LangPack.

More to come...

