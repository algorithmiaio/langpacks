extern crate algorithmia;

use algorithmia::*;
use algorithmia::algo::*;

// This function signature must remain unchanged to interop with the runner
// All other implementation details here may be changed as desired
pub fn apply<'a>(input: AlgoInput<'a>) -> Result<AlgoOutput, Box<std::error::Error>> {
    match input {
        AlgoInput::Text(_text) => Ok(__ALGO__::apply(_text).into()),
        AlgoInput::Json(_json) => Err("Unsupported input type: json".into()),
        AlgoInput::Binary(_bytes) => Err("Unsupported input type: bytes".into()),
    }
}

mod __ALGO__ {
    pub fn apply(name: &str) -> String {
        format!("Hello {}", name)
    }
}
