# Rust-dynamimc-1.x

This was an experiment at building a runner and algorithm separately.

The basic idea is that the algorithm builds into a dynamic lib (.so on *nix)
and the runner is a small binary that dynamically loads the lib, and calls into it.

While I really like the flexibility of allowing very minimalist `apply` methods, 
the trouble with this approach, obvious now in hindsight, is that it doesn't guarantee
that the runner is calling a method that actually exists. The exposed symbols from the C ABI
don't provide enough context to safely determine if we can call a specific method,
so the runner is at risk of causing a segfault, or worse if the input is crafted well enough to execute arbitrary 
code as if it were the algorithm.

It may be possible to mitigate these concerns with a custom build process that injects an
`apply` wrapper function that guarantees a particular ABI is exposed, but short of some hackish
regex code introspection, it seems cleaner to just build the runner directly against the algorithm,
so this LangPack approach is probably destined for a graveyard already.
