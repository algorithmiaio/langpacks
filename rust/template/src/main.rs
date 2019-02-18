use algorithmia::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;


#[derive(Deserialize)]
pub struct Input {
    name: String,
}

#[derive(Serialize)]
struct Output {
    msg: String,
}

fn apply(input: Input) -> Result<Output, Box<dyn Error>> {
    Ok(Output {
        msg: format!("Hello {}", input.name),
    })
}

fn main() {
    // Setup a handler to process API calls
    // For more details, see algorithmia.com/developers/algorithm-development/languages/rust
    handler::run(apply);
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
