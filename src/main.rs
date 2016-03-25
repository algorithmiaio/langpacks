#[macro_use]
extern crate hyper;
extern crate base64;
extern crate serde;
extern crate serde_json;
extern crate time;

use hyper::client::Client;
use hyper::header::ContentType;
use hyper::server::{Handler, Server};
use hyper::Url;
use std::env;

macro_rules! s { ($x:expr) => ($x.to_string()); }
macro_rules! jsonerr { ($x:expr) => (concat!(r#"{"error":{"message":""#, $x, r#""}}"#).to_owned()); }
macro_rules! jsonres { ($x:expr) => (concat!(r#"{"result":""#, $x, r#""}"#).to_owned()); }

mod langserver;
pub mod langrunner;
use langserver::{LangServer, LangServerMode};


fn main() {
    // Configure LangServer to respond sync (block until algo complete) or async (POST algo result back to URL)
    let mode = match env::var("NOTIFY_REQUEST_COMPLETE") {
        Ok(notify_var) => match Url::parse(&notify_var) {
          Ok(url) => LangServerMode::Async(url),
          Err(err) => panic!("Failed to parse NOTIFY_REQUEST_COMPLETE as URL: {}", err),
        },
        Err(env::VarError::NotPresent) => LangServerMode::Sync,
        Err(err) => panic!("Failed to parse NOTIFY_REQUEST_COMPLETE as URL: {}", err),
    };

    // Start LangPack runner and server
    let handler = LangServer::new(mode);
    let listener = Server::http("0.0.0.0:3000").unwrap().handle(handler).unwrap();
    println!("Listening on port 3000.");

    // Optionally notify another service that the LangServer is alive and serving requests
    if let Ok(url) = env::var("NOTIFY_STARTED") {
        if let Err(err) = Client::new()
                                 .post(&url)
                                 .header(ContentType::json())
                                 .body(&jsonres!("Started"))
                                 .send() {
          println!("Failed to send notification that langserver started: {}", err);
        }
    }

    // TODO: on sigterm, close listener, let listener guard fall out of scope, wait_with_timeout for child to exit
    // listener.close();
}
