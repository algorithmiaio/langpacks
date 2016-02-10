# Rust LangPack

This LangPack aims takes a safer approach than they [rust-dynamic](../rust-dynamic-1.x) experiment.
By building the runner directly against the algorithm, we avoid an entire class of segfaults and memory exploits.

This takes much better advantage of type interaction between the runner and algorithm, and results in a very small binary
with virtually zero overhead, but has 2 less-than-ideal characteristics:

1. It adds extra boilerplate to the algorithm, which has to be more explicit about how to handle each type of input.
  - It could be possible to introspect the apply method signature at build (build.rs), to generate a `main.rs` that removes the need for this boilerplate.
2. Changing the apply method signature will result in compile errors that look like they come from the provided `main.rs`
  - I currently think we should refresh/overwrite `main.rs` on every build, to ensure proper handling of the input
