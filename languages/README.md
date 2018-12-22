There is new work in progress to revamp a bit about how languages are created, the base for language images, and how algorithms are compiled.

Here are the current changes (reflected in the templates in this folder and the libraries folder above):
Previously, it was expected that any base-image needed to have langserver installed, along with a language runtime, etc.  This was made easier by using the bin/build-language script.

Now, the base image for a builder or runner can be totally arbitrary and the Dockerfile templates here expect the following:
1. We build a docker image with langserver in it and have it published somewhere (see Dockerfile.langserver)
2. Base builder and runner templates assume that one of the packages installed is an appropriate language build or run time for the algorithm

Language implementations should do the following:
1. Create a directory in languages/<language_name>

2. A directory called `template` that should be a templatized version of the root directory of a newly created algorithm.

**`/template` requirements:**

- The default algorithm generated from the template is called `apply` and is a basic `"Hello $input"` algorithm that compiles and runs without any user modification. This algorithm should be in a file called `__ALGO__.$language_extension`, e.g. `__ALGO__.java` or `__ALGO__.py`
- The template algorithm depends on and imports the Algorithmia client for that language
- The template directory should contain an `algorithmia.conf` file, a Configuration file that must include setting `"username"` as `"__USER__"`, `"algoname"` as `"__ALGO__"`, and `language` as the language. The `username` and `algoname` fields will be replaced when the algorithm is actually created.

3. Create a config.json that lists the artifacts that need to get copied from the builder image into the runner image during algorithm compile

Languages also need:
1. A package to install the buildtime (which means installing a bin/build executable and bin/test executable) in libraries/<some_name>

**bin/build requirements:**
- Use languages most popular package manager to install any any dependencies needed
- Compile any code, if needed
- Exit with 0 if successful and nonzero otherwise

**bin/test requirements:**
- Launch a test runner that uses the languages most popular testing framework and tests the templated `__ALGO__` algorithm with a single test that passes
- Exit with 0 if successful and nonzero otherwise


2. A package to install the runtime (which is a bin/pipe executable) in libraries/<some_name>

bin/pip requirements:
- In depth explanation of the contract [can be found here](https://github.com/algorithmiaio/langpacks/blob/master/langpack_guide.md) but basically it's an executable that expects the user's request to come from stdin and writes errors and the algorithms response to stderr and `/temp/algoout`. Often, the typical way to provide an executable with this contract is to write a 'helper' pipe program in the actual language that turns untyped stdin into appropriate input for your language, and writes return values to `/temp/algoout`. Then the actual bin/pipe is able to provide it's real contract by piping into and calling this program. Having a wrapper executable isn't always necessary, languages like rust and C who's only runtime is libc (which is already on every UNIX system) will turn into executables when their pipe.rs or pipe.c files are compiled.
