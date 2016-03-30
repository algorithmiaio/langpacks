#[macro_use]
extern crate hyper;
#[macro_use]
extern crate quick_error;

extern crate base64;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate wait_timeout;

use hyper::server::{Handler, Server};
use std::env;
use time::{Duration, PreciseTime};
use std::error::Error as StdError;

macro_rules! s { ($x:expr) => ($x.to_string()); }
macro_rules! jsonerr { ($x:expr) => (concat!(r#"{"error":{"message":""#, $x, r#""}}"#).to_owned()); }

mod langserver;
pub mod error;
pub mod langrunner;
pub mod notifier;

use error::Error;
use langserver::{LangServer, LangServerMode};
use notifier::{Notifier, LoadNotification, LoadStatus};

fn main() {
    let start = PreciseTime::now();

    let listener = get_mode().and_then(|mode| {
        // Start LangPack runner and server
        let lang_server = LangServer::new(mode);
        let listener = Server::http("0.0.0.0:3000")
                              .and_then(|s| s.handle(lang_server));
        println!("Listening on port 3000.");
        listener.map_err(|err| err.into())
    });

    let duration = start.to(PreciseTime::now());

    match listener {
        Ok(mut listener) => {
            let _ = load_complete(LoadStatus::Success, duration)
                .or_else(|_| listener.close());
        }
        Err(ref err) => {
            println!("Failed to load: {}", err);
            let status = LoadStatus::Failure(err.description().to_owned());
            let _ = load_complete(status, duration);
        }
    };
}


fn load_complete(status: LoadStatus, duration: Duration) -> Result<(), Error> {
    // Optionally notify another service that the LangServer is alive and serving requests
    if let Ok(url) = env::var("LOAD_COMPLETE") {
        let notifier = Notifier::parse(&url).expect("LOAD_COMPLETE not a valid URL");

        let message = LoadNotification::new(&status, duration);
        try!(notifier.notify(message, None));
    }
    Ok(())
}

// Mode determines if request should until algo complete (Sync)
//   or POST algo result back to URL when algo starts (Async)
fn get_mode() -> Result<LangServerMode, Error> {
    match env::var("REQUEST_COMPLETE") {
        Ok(url) => {
            let notifier = Notifier::parse(&url).expect("REQUEST_COMPLETE not a valid URL");
            Ok(LangServerMode::Async(notifier))
        }
        Err(env::VarError::NotPresent) => Ok(LangServerMode::Sync),
        Err(err) => Err(err.into()),
    }
}

