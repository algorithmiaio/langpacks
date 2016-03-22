# LangPacks

This is work-in-progress tracking the [LangPack spec proposal](https://docs.google.com/a/algorithmia.io/document/d/1vd80VKXX5kPIYIHpaXV-oD15aw2CyoM6vXWQLTH9MfI/edit?usp=sharing)

The current goal of this repo is to flush out any issues with the spec proposal,
and demonstrate the viability of the spec with 2 reference implementations:
- one dynamic scripting language (currently ruby)
- one strongly-typed non-JVM compiled language lacking (currently rust)

----

LangPack: A language specific package the encompasses language-specific system deps, a language-specific way to manage algorithm deps, and the ability to run a language-specific algorithm.

LangServer: A server that emulates a light-weight version of the Algorithmia API for any `bin/pipe` that adheres to the LangPack spec.

## Building LangServer(s)

Disclaimer: The intent was to prototype langserver in rust (because I knew it better), but finally write it in go (lower barrier to entry), but it turned into an official project before the rewrite happened. So, for now: start by installing [latest stable rust](https://www.rust-lang.org/downloads.html), and then:

```
cargo build --release
docker build -t algorithmia/langserver .
```

Then the container for individual containers can be built

```
docker build -t algorithmia/langserver-ruby ruby-2.x
```

## Running LangServer

In production: bind mount algorithm.zip into `/algorithmia`

```
docker run --rm -it -v /path/to/algorithm.zip:/algorithmia/ -p 3000:3000 algorithmia/langserver-<lang>
```

In runner/algorithm development: bind mount algorithm dir into `/algorithm` (not `/algorithmia`)

```
docker run --rm -it -v `pwd`:/algorithm -p 3000:3000 algorithmia/langserver-<lang>
```

## Wanna contribute?

- Fix anything that's broke.
- Add comments to the proposal.
- Propose alternatives.

Bonus 🌮🌮tacos🌮🌮 for you if you write even a single line of code for any experimental LangPack.


