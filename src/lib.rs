#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate base64;
extern crate hyper;
extern crate libc;
extern crate serde;

extern crate wait_timeout;

macro_rules! s { ($x:expr) => ($x.to_string()); }
macro_rules! printerrln {
    ($($arg:tt)*) => ({
        use std::io::prelude::*;
        if let Err(e) = write!(&mut ::std::io::stderr(), "{}\n", format_args!($($arg)*)) {
            panic!("Failed to write to stderr.\
                \nOriginal error output: {}\
                \nSecondary error writing to stderr: {}", format!($($arg)*), e);
        }
    })
}

pub use langserver::{LangServer, LangServerMode};

mod langserver;
pub mod error;
pub mod langrunner;
pub mod notifier;
pub mod message;
