# Package set validation

The `environment_validator.py` tool allows you to verify if a new package & template work as expected before deploying to test.

- The validator allows you to select either a language or dependency based template file for debugging purposes.
- The validator follows the same process that legit does when it instances and creates an algorithm.
- The validator builds all required docker images; the base image, builder and runner images, and then combines all of them in the compile image.
- This compile image is how, in IPA - we compile algorithm source code and execute that algorithm in a runner image.

## Dependencies

System dependencies:

- libssl-dev
- libcurl4-openssl-dev

Ensure that python dependencies in requirements.txt are installed

```
pip install -r requirements.txt
```

## How to execute

**Important**: Execute the `environment_validator.py` from the root directory of langpacks:

```
./tools/environment_validator.py -h
```

### Parameter definitions

```
$ ./tools/environment_validator.py -h
usage: environment_validator.py [-h] [-b BASE_IMAGE] [-g LANGUAGE_GENERAL_NAME]
                               -s LANGUAGE_SPECIFIC_NAME -t TEMPLATE_TYPE -n
                               TEMPLATE_NAME [-d DEPENDENCIES] [-c CLEANUP]
                               [--local-dependency-src LOCAL_SRC]
                               [--local-dependency-dest LOCAL_DEST]

Creates a simulation of the IPA / langserver / algorithm interface. Use this
to test new language, and new dependency packages.

optional arguments:
  -h, --help            show this help message and exit
  -b BASE_IMAGE, --base-image BASE_IMAGE
                        the linux base image to build your packageset on top
                        of. Usually an ubuntu version.Defaults to
                        'ubuntu:16.04'
  -g LANGUAGE_GENERAL_NAME, --language-general-name LANGUAGE_GENERAL_NAME
                        The general name for your language, if multiple minor
                        versions can use the same runtime/buildtime.For
                        example: Python3 or Python2.Defaults to the value
                        defined for --language-specific-name
  -s LANGUAGE_SPECIFIC_NAME, --language-specific-name LANGUAGE_SPECIFIC_NAME
                        The fully specified name of your language.For example:
                        Python37. or csharp-dot-core2.
  -t TEMPLATE_TYPE, --template-type TEMPLATE_TYPE
                        The type of template we're using, this can be
                        either:'dependency' - for frameworks/etc 'language' -
                        for new language implementations & modifications
  -n TEMPLATE_NAME, --template-name TEMPLATE_NAME
                        The name of your template directory.For example:
                        pytorch-1.0.0, orjava11.
  -d DEPENDENCIES, --dependency DEPENDENCIES
                        A list builder of all non-language dependency packages
                        that your algorithm needs.Language core, buildtime &
                        runtime are included automatically.
  -c CLEANUP, --clean-up CLEANUP
                        A boolean variable that if set, forces us to clean up
                        docker containers and images created by this process.
  --local-dependency-src LOCAL_SRC
                        If using a local cached dependency for testing, is the
                        path towards that dependency on your file system.
  --local-dependency-dest LOCAL_DEST
                        If using a local cached dependency for testing, is the
                        path where the dependency will live in the
                        compileLocal image.

```

## Testing

Once you see that langserver is listening on port `9999`, you can send a `curl` request in another terminal.
example:

```
curl localhost:9999 -H 'Content-Type: application/json' -d '{"name": "Anthony"}'
```

If the response object is as expected, then the langserver, algorithm interface is working as expected! Congrats! :+1:

