#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate log;

extern crate base64;
extern crate hyper;
extern crate libc;
extern crate serde;

extern crate wait_timeout;

macro_rules! s { ($x:expr) => ($x.to_string()); }

pub use langserver::{LangServer, LangServerMode};

mod langserver;
pub mod error;
pub mod langrunner;
pub mod notifier;
pub mod message;
