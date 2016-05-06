extern crate algorithmia;
extern crate rustc_serialize;

use algorithmia::*;
use algorithmia::algo::*;
use algorithmia::data::*;

#[derive(Default)]
pub struct Algo;

// Algo should implement EntryPoint or DecodedEntryPoint
//   and override at least one of the apply method variants
impl EntryPoint for Algo {
    fn apply_str(&self, input: &str) -> Result<AlgoOutput, Box<std::error::Error>> {
        let msg = format!("Hello {}", input);
        Ok(AlgoOutput::Text(msg))
    }
}