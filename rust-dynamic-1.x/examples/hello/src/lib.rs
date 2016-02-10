extern crate algorithmia;

use algorithmia::*;
use algorithmia::algo::*;

//
// Rust algorithms must implement one of these 3 methods:
//   apply(input: &str) -> Result<AlgoResult, String>
//   apply(input: JSON) -> Result<AlgoResult, String>
//   apply(input: &[u8]) -> Result<AlgoResult, String>


#[no_mangle]
pub fn apply(input: &str) -> Result<AlgoResult, String> {
    let greeting = format!("Hello {}", input);

    // Could also do: Ok(greeting.into())
    Ok(AlgoResult::Text(greeting))
}
