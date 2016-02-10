extern crate algorithmia;

use algorithmia::*;
use algorithmia::algo::*;

// Boilerplate for interop with the runner
// This function signature must remain unchanged
pub fn apply<'a>(input: AlgoInput<'a>) -> Result<AlgoResult, String> {
    match input {
      AlgoInput::Text(_text) => Ok(hello::greet(_text).into()),
      AlgoInput::Json(_json) => Err("Unsupported input type: json".into()),
      AlgoInput::Binary(_bytes) => Err("Unsupported input type: bytes".into()),
    }
}

mod hello {
  pub fn greet(name: &str) -> String {
    format!("Hello {}", name)
  }
}
