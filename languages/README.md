There is new work in progress to revamp a bit about how languages are created, the base for language images, and how algorithms are compiled.

Here are the current changes (reflected in the templates in this folder and the libraries folder above):
Previously, it was expected that any base-image needed to have langserver installed, along with a language runtime, etc.  This was made easier by using the bin/build-language script.

Now, the base image for a builder or runner can be totally arbitrary and the Dockerfile templates here expect the following:
1. We build a docker image with langserver in it and have it published somewhere (see Dockerfile.langserver)
2. Base builder and runner templates assume that one of the packages installed is an appropriate language build or run time for the algorithm

Language implementations should do the following:
1. Create a directory in languages/<language_name>
2. Create a basic template for what an algorithm in that language should look like in languages/<language_name>/template (this should include an algorithmia.conf file but that is the only requirement)
3. Create a config.json that lists the artifacts that need to get copied from the builder image into the runner image during algorithm compile

Languages also need:
1. A package to install the buildtime (including a bin/build script and bin/test script) in libraries/<some_name>
2. A package to install the runtime (including a bin/pipe script) in libraries/<some_name>
  a. Some languages might not need a bin/pipe script, for example a statically compiled language like rust will create an artifact in bin/build called pipe that will be copied over
  b. Some languages might not have a runtime at all (e.g. static binary compiled languages)
