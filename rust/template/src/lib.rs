extern crate algorithmia;

#[allow(unused_imports)]
use algorithmia::*;
use algorithmia::algo::*;

// This function signature must remain unchanged to interop with the runner
pub fn apply<'a>(input: AlgoInput<'a>) -> Result<AlgoOutput, Box<std::error::Error>> {
    // Use .as_string, .as_json, or .as_bytes, depending on the input you want to support
    match input.as_string() {
        Some(text) => Ok(__ALGO__::apply(&text).into()),
        None => Err("Unsupported input type".into())
    }
}

mod __ALGO__ {
    pub fn apply(name: &str) -> String {
        format!("Hello {}", name)
    }
}
