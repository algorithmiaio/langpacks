
# LangPacks

*LangPack*: A language-specific package the encompasses steps to setup, build, and run language-specific algorithms.

*LangServer*: A server that serves a LangPack's `bin/pipe` runner in a way that emulates a light-weight version of the Algorithmia API.

## Process for Creating a new LangPack

1. [File a tracking issue](https://github.com/algorithmiaio/langpacks/issues/new?title=New+LangPack:+LANGUAGE).
   - We want to avoid competing implementations of a LangPack in the short term
2. Implement an Algorithmia client library - (TODO: add the client SDK guide to this repo)
   - LangPacks depend on an Algorithmia client to support calling other algorithms and managing data
   - Some languages may get away with reusing existing clients (e.g JVM-based languages can use the Java client)
3. Fork [algorithmiaio/langpacks](https://github.com/algorithmiaio/langpacks) and add a directory with your LangPack implementation.
4. Create a pull request for your LangPack. Algorithmia will review and test it before merging.
5. Once merged, Algorithmia will deploy it and make it available for beta testers (details shared on the tracking issue).
6. Once adequately tested, it will be released publicly and announced in the newsletter and/or blog.


## Implementing a LangPack

A LangPack is a template directory that contains the following files and directory structure.

path                | Description
------------------- | ---------------
template/           | directory containing templatized versions of an algorithm’s initial files and structure
template/bin/setup  | executable that installs any language build or runtime dependencies (e.g. compiler, runtime)
template/bin/build  | executable that builds an algorithm capable of being run by this LangPack
template/bin/test   | executable that runs an algorithm's tests
template/bin/pipe   | executable that runs an algorithm built by this LangPack (may be in the template or a built artifact)
examples/           | any example algorithms - used for testing langpacks during development


## `template/`

This directory should be a templatized version of the root directory of a newly created algorithm.

**Requirements:**

- The default algorithm generated from the template is a basic `"Hello $input"` algorithm that compiles and runs without any user modification
- The template algorithm depends on and imports the Algorithmia client for that language
- TODO: It includes a basic `README.md` (e.g. marketplace URL, badge, how to build/run locally)

The template directory may contain `__username__` or `__algoname__` anywhere
in paths, filenames, or contents of files and they will be replaced when a new algorithm is created.
There is a [`make-example`](https://github.com/algorithmiaio/langpacks/blob/master/bin/make-example)
that emulates this templatization process.


## `bin/setup`

executable that installs any language build or runtime dependencies (e.g. compiler, runtime)

**Requirements:**

1. Install any dependencies required by the `bin/build` (e.g. compiler) or `bin/pipe` (e.g. runtime).
   - Note: If dependencies required for bin/build are different than the dependencies required for bin/pipe, then this script may check for the presence of a --runtime flag
2. Exit with code 0 if successful, otherwise nonzero.

**Testing:**

*TODO: PUBLISH THESE MINIMAL BASE CONTAINERS*

Algorithmia provides the base build and run containers on Docker Hub, so verifying bin/install-build-deps is just a matter of verifying these commands run and complete successfully inside those containers:

```
docker run -it -v `pwd`/bin:/algo/bin algorithmia/langpack-base /algo/bin/setup
docker run -it -v `pwd`/bin:/algo/bin algorithmia/langpack-base /algo/bin/setup --runtime
```

## `bin/build`

**Requirements:**

1. Fetch and cache algorithm dependencies
   - Use the language's most popular package manager
   - The backend may need to be aware of how dependency caching works to optimize build times
2. Compile any code, if needed
3. Zip up any files needed to run into `algorithmia.zip` in the working directory
   - This zip must include `bin/pipe` at that location within the zip
   - Any algorithm runtime dependendencies (i.e. installed by package manager) must also be in the zip
4. Exit with 0 if successful, otherwise nonzero

**Testing:**

TODO: explain creating a docker single-language image that can bind mount the algorithm directory and run `bin/build`

## `bin/test`

**Requirements:**

1. Must launch a test runner for the language. It should default to the most-commonly preferred test framework for the lanaguage.
2. Must output test results to STDOUT/STDERR
3. Exit with 0 if successful, otherwise nonzero

**Testing:**

The default templated algorithm should include a single test that passes by default.


## `bin/pipe`

`bin/pipe` is an executable that executes an algorithm when it is run. Communitaion to and from `bin/pipe` includes:

- stdin: Per-line compact JSON-serialized `Request` objects containing structured views of the API request payload
- stdout/stderr: Reserved for piping algorithm stdin and stderr back to the LangServer
- algoout FIFO (`/tmp/algoout`): a named pipe for writing algorithm output to. Write to it as if it were a file.

**Requirements:**

- Write `"PIPE_INIT_COMPLETE\n"` to STDOUT when the algorithm is successfully loaded
- Read and decode a single line of compact JSON on STDIN. New lines will be the separator between separate inputs.
- Convert STDIN into the appropriate argument(s) for invoking the algorithm
  - `bin/pipe` will be run from the root of the extracted `algorithm.zip` directory.
- Wait for the algorithm to complete
- Flush stdout (this should be the only time the runner touches stdin/stdout)
- Write the algorithm result or error to the algoout pipe per the output definition section.
- Repeat until STDIN reaches EOF
- Exit with 0 if successful, otherwise nonzero

Note: For native compiled languages, `bin/pipe` may be a binary executable that was built by `bin/build` directly against the algorithm, in which case bin/pipe might only exist in the algorithm.zip.

**Request object:**

The Request object is received by `bin/pipe` on stdin. It is a single line of compact JSON containing 2 fields:

Field          | Type   | Description
---------------|--------|-------------
`content_type` | String | Defines the format of the `data` field: `text`, `json`, or `binary`
`data`         | Varies | The input data for the algorithm

Parsing of the `data` field is determined by the `content_type` value:

- `text`: data is a string (UTF-8)
- `json`: data is valid JSON (UTF-8)
- `binary`: data is base64-encoded bytes

Example `Request` objects (note: rendered as pretty JSON here, but `bin/pipe` will always receive them as a single line of compact JSON).

Text input:
```
{
  "content_type": "text",
  "data": "I am definitely a mad man with a box!"
}
```

JSON input:
```
{
  "content_type": "json",
  "data": ["Bigger on the inside.", 42, true, {"smith": "pond", "capaldi": "oswald"}]
}
```

Binary input:
```
{
  "content_type": "binary",
  "data": "RGVtb25zIHJ1biB3aGVuIGEgZ29vZCBtYW4gZ29lcyB0byB3YXIuCk5pZ2h0IHdpbGwgZmFsbCBhbmQgZHJvd24gdGhlIHN1biwgCldoZW4gYSBnb29kIG1hbiBnb2VzIHRvIHdhci4gCkZyaWVuZHNoaXAgZGllcyBhbmQgdHJ1ZSBsb3ZlIGxpZXMsIApOaWdodCB3aWxsIGZhbGwgYW5kIHRoZSBkYXJrIHdpbGwgcmlzZSwgCldoZW4gYSBnb29kIG1hbiBnb2VzIHRvIHdhci4gCkRlbW9ucyBydW4sIGJ1dCBjb3VudCB0aGUgY29zdC4gClRoZSBiYXR0bGUncyB3b24gYnV0IHRoZSBjaGlsZCBpcyBsb3N0Lgo="
}
```

**Output:**

If no error is encountered, the return value should written written directly to the `algoout` pipe as compact JSON with a content type.
- String results can be written out as-is and set `content_type` to `"text"`
- Objects that can be serialized to JSON should be written out as JSON and set `content_type` to `"json"`
- Byte arrays/streams should write out the base64 encoding and set `content_type` to `"binary"`

```
{
    "result": ALGORITHM_RESULT,
    "content_type": "text|json|binary"
}
```

If an error is encountered, the error message, [optional] stacktrace and error_type should be written to the `algoout` pipe as compact JSON in the following format:

```
{
    "message": "The algorithm exploded like the TARDIS",
    "stacktrace": "VanGoughException occurred in /algo/src/ExplodingTardis.py:1890\n\tin /algo/src/PandoricaOpens.py:512",
    "error_type": "VanGoughException"
}
```

Note: Writing output as compact JSON is acceptable but not required.

**Testing:**

Testing a `bin/pipe` script involves a few steps:

1. Build a `langserver-LANGUAGE` and `langbuilder-LANGUAGE` images
   - From the root of the langpack repo, run: `bin/build LANGUAGE`
2. Generate an algorithm from your template
   - From the root of the langpack repo, run: `bin/make-example LANGUAGE`

3. Build algorithm with `langbuilder-LANGUAGE` image
   - TODO: explain

4. Run LangServer image with your generated algorithm bind-mounted (see "Running Langserver" below)

   - Note: Make sure you use the absolute path to the algorithm.zip.

```
docker run --rm -it -v /path/to/algorithm.zip:/tmp/algorithm.zip -p 9999:9999 algorithmia/langserver-LANGUAGE
```

5. `POST` to the langserver API on localhost:9999 with a `Content-Type` header:
   - `curl localhost:9999 -X POST -H 'Content-Type: text/plain -d 'testing'`
   - `curl localhost:9999 -X POST -H 'Content-Type: application/json -d '{"foo", "bar"}'`
   - `curl localhost:9999 -X POST -H 'Content-Type: application/x-octet-stream --data-binary @cats.jpg`


## Advanced LangServer Usage

The `init-langserver` script in langserver containers provides 2 ways to run an algorithm:

#### Bind mount algorithm.zip to /tmp/algorithm.zip
Note: Make sure you use the absolute path to the algorithm.zip.
```
docker run --rm -it -v /path/to/algorithm.zip:/tmp/algorithm.zip -p 9999:9999 algorithmia/langserver-<lang>
```

#### Bind mount algorithm directory to /tmp/algorithm
```
docker run --rm -it -v `pwd`:/tmp/algorithm -p 9999:9999 algorithmia/langserver-<lang>
```
