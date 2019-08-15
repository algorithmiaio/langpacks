# LangPacks

*LangPack*: A language specific package the encompasses steps to setup, build, and run language-specific algorithms.

*LangServer*: A server that serve a LangPack's `bin/pipe` runner in a way that emulates a light-weight version of the Algorithmia API.

## LangServer

Langserver could be simplified like this: it's about 1000 lines of code that emulates a simple API that looks/feels like our API server's API for calling an algo (lacking features like auth)

It translates that HTTP standard into a STDIO-based input and a named pipe for output (defined in the langpack_guide.md)
- it was important that multiple subsequent requests could reuse the same process
- The focus was a standard that every language could easily implement. Having each language implement a web-server was considered, but there was uncertainty about how easy that would be for langs like R, and if we wanted to alter how it integrates with the rest of the backend, it'd need reimplemented multiple times (e.g. if we wanted to expose stdout/stderr via websockets, it would only need implemented in langserver, not each langpack)
- I also considered other queues, e.g. posix message queues, but struggled to gain confidence it would work well for all languages (R again being a concern)
- Ultimately, every language has very simple ways of interacting with files (incl stdio and named pipes), so stdio and a named pipe (fifo) were chosen for the simplicity to work with them in any lang. The fifo allowed us to leave stdout/stderr in tact.

Weirder details:
1) Langserver spins up 2 threads that collect stdout/stderr and recombined them into the result.
2) Langserver has 2 modes: sync vs async. Basically, sync is how it was originally built, to be easy to debug as a simple web-server that looks like the API server. `async` mode was added to make it integrate with dockherder, so that dockherder could call it and forget about it until a callback informed dockherder that it was complete.

## Building LangServer(s) (Partially deprecated)

Disclaimer: The intent was to prototype langserver in rust (because I knew it better), but finally write it in go (lower barrier to entry), but it turned into an official project before the rewrite happened. So, for now: start by installing [latest stable rust](https://www.rust-lang.org/downloads.html), and then:

```
bin/build langserver     # just builds the base langserver images (default)
bin/build <lang>         # builds language-specific image (and deps)
bin/build all            # builds all images for all langpacks
bin/build single-runner  # builds 1 image containing the langserver runner and running setup on all langpacks
bin/build single-builder # builds 1 image containing the langserver builder and running setup on all langpacks
bin/build single         # builds the single-runner and single-builder
```

Note: the initial plan is to NOT use these images, but they are helpful for implementing and testing langpacks locally, as well as provide some "code documentation" for how setup/build/pipe/langserver all fit together.

## Building LangServer with Libraries
We're in the process of refactoring the way that images get generated and algorithms are compiled.  The initial approach would create an `algorithm.zip` that contains a compiled binary (or source for interpreted languages) along with any dependencies needed.  Additionally, a number of libraries were installed side-by-side which made it difficult to debug in certain scenarios or independently evolve various languages.  In particular, some libraries required certain variables set during install/compilation but not during execution and it was difficult to determine what variables or even system packages were needed for what libraries in particular.

The new process (still experimental) involves templating a Dockerfile based on a set of desired `libraries` (which could be language runtimes/buildtimes, services, or deep-learning frameworks) and then building an image with just that subset of libraries.  Ideally, libraries' install.sh script should be able to run on an Ubuntu 16.04 host/VM the same as it could during docker build time (this greatly eases creating the install script).

Algorithms no longer have a single `bin/build` script but two separate scripts, one to `install-dependencies` (which would do an appropriate pip/npm/cargo/etc install/fetch) and one to `install-algorithm` which compiles or bundles the algorithm source to `/opt/algorithmia`.

Templating and building a dockerfile:
```
$ ./bin/build-template --help
usage: build-template [-h] [-l LIBRARY] [-p TEMPLATE] -t TAG [-o OUTPUT]
                      [-u USER_ID]

Creates a dockerfile, templating in any needed files and environment variables
to set up different libraries. Libraries will be installed _in order
specified_ so if one needs to be installed before another, then list them in
that order on the command line

Will then run a docker build and tag operation

Library directories should include the following:
  - install.sh : a script to install the library
  - config.json (optional): a json file containing configuration such as:
    - env_variables: dictionary of environment variables to
      set at the end of execution
    - install_scripts: list of order to run scripts in to create
      multiple layers (particularly for testing)

optional arguments:
  -h, --help            show this help message and exit
  -l LIBRARY, --library LIBRARY
                        library directories to include in generating this
                        dockerfile
  -p TEMPLATE, --template TEMPLATE
                        location of the dockerfile template file
  -t TAG, --tag TAG     tag to label the docker image once produced
  -o OUTPUT, --output OUTPUT
                        name of file to write output to
  -u USER_ID, --user_id USER_ID
                        user id to use for the "algo" user, defaults to
                        current user
```
Examples:
```
# Create a langpack consisting only of python2 and tag it as algorithmia/langpack-runner:python2
./bin/build-template -u 1001 -t algorithmia/langpack-runner:python2 -l python2 -o docker/templated/Dockerfile.python2

# Create a langpack with NVIDIA GPU drivers, python and caffe and tag it as algorithmia/langpack-runner:python2-caffe
./bin/build-template -u 1001 -t algorithmia/langpack-runner:python2-caffe -l gpu-driver -l python2 -l caffe -o Dockerfile.python2-advanced
```
### Building an algorithm
1. Bind mount an algorithm working directory to `/tmp/build` - `docker run -it -v \`pwd\`:/tmp/build algorithmia/langpack-runner:python2`
2. Run `/tmp/build/bin/install-dependencies`
3. Run `/tmp/build/bin/install-algorithm`
4. Outside of the container commit the image with appropriate entrypoint - `docker commit -c 'ENTRYPOINT /bin/init-langserver' -c 'WORKDIR /opt/algorithm' <container_id> algorithmia/<algorithm_name>`

### Running an algorithm
1. `docker run --rm -ti -p 9999:9999 algorithmia/<algorithm_name>`

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

