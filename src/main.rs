#[macro_use]
extern crate hyper;

extern crate base64;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate wait_timeout;

use hyper::server::{Handler, Server};
use std::env;
use time::PreciseTime;

macro_rules! s { ($x:expr) => ($x.to_string()); }
macro_rules! jsonerr { ($x:expr) => (concat!(r#"{"error":{"message":""#, $x, r#""}}"#).to_owned()); }
macro_rules! jsonres { ($x:expr) => (concat!(r#"{"result":""#, $x, r#""}"#).to_owned()); }

mod langserver;
pub mod langrunner;
pub mod notifier;
use langserver::{LangServer, LangServerMode};
use notifier::{Notifier, LoadNotification, LoadStatus};

fn main() {
    let start = PreciseTime::now();

    // Configure LangServer to respond sync (block until algo complete) or async (POST algo result back to URL)
    let mode = match env::var("REQUEST_COMPLETE") {
        Ok(url) => {
            let notifier = Notifier::parse(&url).expect("REQUEST_COMPLETE not a valid URL");
            LangServerMode::Async(notifier)
        }
        Err(env::VarError::NotPresent) => LangServerMode::Sync,
        Err(err) => panic!("Failed to parse REQUEST_COMPLETE as URL: {}", err),
    };

    // Start LangPack runner and server
    let lang_server = LangServer::new(mode);
    let mut listener = Server::http("0.0.0.0:3000").unwrap().handle(lang_server).unwrap();
    let duration = start.to(PreciseTime::now());
    println!("Listening on port 3000.");

    // Optionally notify another service that the LangServer is alive and serving requests
    if let Ok(url) = env::var("LOAD_COMPLETE") {
        let notifier = Notifier::parse(&url).expect("LOAD_COMPLETE not a valid URL");
        // TODO: fix error handling and don't always send Success
        let message = LoadNotification::new(LoadStatus::Success, duration);
        if let Err(err) = notifier.notify(message, None) {
            println!("Failed to send LOAD_COMPLETE notification: {}", err);
            let _ = listener.close();
        }
    }
}