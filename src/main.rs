#[macro_use]
extern crate quick_error;

extern crate base64;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate wait_timeout;

use hyper::server::{Handler, Server};
use std::env;
use std::time::{Duration, Instant};

macro_rules! s { ($x:expr) => ($x.to_string()); }

mod langserver;
pub mod error;
pub mod langrunner;
pub mod notifier;
pub mod message;

use error::Error;
use langserver::{LangServer, LangServerMode};
use notifier::{Notifier, HealthStatus};
use message::StatusMessage;

fn main() {
    let start = Instant::now();

    let listener = get_mode().and_then(|mode| {
        // Start LangPack runner and server
        let lang_server = LangServer::new(mode, get_status_notifier());
        let listener = Server::http("0.0.0.0:9999").and_then(|s| s.handle(lang_server));
        println!("Listening on port 9999.");
        listener.map_err(|err| err.into())
    });

    let duration = start.elapsed();

    match listener {
        Ok(mut listener) => {
            let _ = load_complete(HealthStatus::Success, duration).or_else(|_| listener.close());
        }
        Err(err) => {
            println!("Failed to load: {}", err);
            let status = HealthStatus::Failure(err);
            let _ = load_complete(status, duration);
        }
    };
}


fn load_complete(status: HealthStatus, duration: Duration) -> Result<(), Error> {
    // Optionally notify another service that the LangServer is alive and serving requests
    if let Some(notifier) = get_status_notifier() {
        let message = StatusMessage::new(status, duration, None, None);
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

fn get_status_notifier() -> Option<Notifier> {
    match env::var("STATUS_UPDATE") {
        Ok(url) => Some(Notifier::parse(&url).expect("STATUS_UPDATE not a valid URL")),
        Err(env::VarError::NotPresent) => None,
        Err(err) => panic!("Error reading STATUS_UPDATE: {}", err),
    }
}