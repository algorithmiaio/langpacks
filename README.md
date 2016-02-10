# LangPacks

This is **experimental**, **exploratory** work-in-progress tracking the [LangPack spec proposal](https://docs.google.com/a/algorithmia.io/document/d/1vd80VKXX5kPIYIHpaXV-oD15aw2CyoM6vXWQLTH9MfI/edit?usp=sharing)

The current goal of this repo is to flush out any issues with the spec proposal,
and demonstrate the viability of the spec with 2 reference implementations:
- one dynamic scripting language (currently ruby)
- one strongly-typed non-JVM compiled language lacking (currently rust)

There are several open questions (in the doc), and neither implementation completely conforms to the current draft.
Nothing is set in stone.

----

## Testing a LangPack

Each LangPack currently has a rudimentary `hello` sample algorithm. Running it is currently something like:

1. Figure out how to install the build deps (cuz the install-deps scripts are empty)
2. Read `bin/build` - figure out what you need to do to build the sample hello app (no need to create algorithm.zip)
3. Create the "algoout" pipe and tail it: `mkfifo /tmp/algoout && tail -f /tmp/algoout`
4. Run the `pipe` executable. It listens on STDIN until EOF (Ctrl-d).
5. Paste a sample request json blob. [text.json](sample-input/text.json) is probably the only one that works right now
6. Maybe, just maybe, if you did all that just right, and if the planets are aligned, you might see a "Hello ..." message on the algout pipe that you're tailing

## Wanna contribute?

- Fix anything that's broke.
- Add comments to the proposal. 
- Propose alternatives.

Bonus 🌮🌮tacos🌮🌮 for you if you write even a single line of code for any experimental LangPack.
