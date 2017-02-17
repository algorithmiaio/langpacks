#[macro_use]
extern crate algorithmia;

use algorithmia::prelude::*;

// API calls will begin at the apply() method, with the request body passed as 'input'
// For more details, see algorithmia.com/developers/algorithm-development/languages
algo_entrypoint!(&str);
fn apply(input: &str) -> Result<String, String> {
    Ok(format!("Hello {}", input))
}


#[cfg(test)]
mod test {
    use super::apply;

    #[test]
    fn test_apply() {
        assert_eq!(&apply("Jane").unwrap(), "Hello Jane");
    }
}
