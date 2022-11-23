use algorithmia::prelude::*;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Declare extern C functions for FFI
extern "C" {
    fn hello(s: *const c_char) -> *const c_char;
    fn cleanup();
}

fn apply(input: CString) -> Result<String, Box<dyn Error>> {
    let output;
    unsafe {
        let output_ptr = hello(input.as_ptr());
        output = CStr::from_ptr(output_ptr)
            .to_string_lossy()
            .into_owned();

        cleanup();
    };

    Ok(output)
}

fn main() {
    handler::run(apply);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_apply() {
        let input = CString::new("Jane").unwrap();
        let expected = CString::new("Hello Jane").unwrap();
        assert_eq!(apply(input).unwrap(), expected);
    }
}
