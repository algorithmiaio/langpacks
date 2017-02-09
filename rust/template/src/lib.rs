#[macro_use]
extern crate algorithmia;

use algorithmia::prelude::*;

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
