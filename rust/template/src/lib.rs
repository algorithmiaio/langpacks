extern crate algorithmia;

#[allow(unused_imports)]
use algorithmia::*;
use algorithmia::algo::*;

#[derive(Default)]
pub struct Algo;

impl AlgoEntryPoint for Algo {
    fn apply_str(&self, name: &str) -> Result<AlgoOutput, Box<std::error::Error>> {
        let msg = format!("Hello {}", name);
        Ok(msg.into())
    }

    // Alternate methods you can override:
    // fn apply_json(&self, json: &Json) -> Result<AlgoOutput, Box<std::error::Error>> {}
    // fn apply_bytes(&self, bytes: &[u8]) -> Result<AlgoOutput, Box<std::error::Error>> {}
    // fn apply<'a>(&self, input: AlgoInput<'a>) -> Result<AlgoOutput, Box<std::error::Error>> {}
}