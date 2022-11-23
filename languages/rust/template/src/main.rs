use algorithmia::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

// Input, Output, and Error types are all configurable
// For more details, see algorithmia.com/developers/algorithm-development/languages/rust

#[derive(Deserialize)]
pub struct Input {
    name: String,
}

#[derive(Serialize)]
struct Output {
    msg: String,
}

// Entry point for each API call
fn apply(input: Input) -> Result<Output, Box<dyn Error>> {
    Ok(Output {
        msg: format!("Hello {}", input.name),
    })
}

// Initial setup
fn main() {
    handler::run(apply)
}

//
// Test case(s)
//
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_apply() {
        let input = Input {
            name: "Jane".to_string(),
        };
        assert_eq!(&apply(input).unwrap().msg, "Hello Jane");
    }
}
