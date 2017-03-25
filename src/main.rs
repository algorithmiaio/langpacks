extern crate langserver;
extern crate hyper;

use hyper::server::Server;
use std::env;
use std::time::{Duration, Instant};

use langserver::error::Error;
use langserver::message::StatusMessage;
use langserver::notifier::Notifier;
use langserver::{LangServer, LangServerMode};

fn main() {
    let start = Instant::now();

    let listener_res = get_mode()
        // Start LangPack runner and server
        .and_then(|mode| { LangServer::start(mode, get_status_notifier()) })
        // Start serving the LangServer handler
        .and_then(|lang_server| {
            Server::http("0.0.0.0:9999")
                .and_then(|s| s.handle(lang_server))
                .map_err(|err| err.into())
        });

    let duration = start.elapsed();

    if listener_res.is_ok() {
        println!("Listening on port 9999.");
    }

    match listener_res {
        Ok(mut listener) => {
            let _ = load_complete(Ok(()), duration).or_else(|_| listener.close());
        }
        Err(err) => {
            println!("Failed to load: {}", err);
            let _ = load_complete(Err(err), duration);
        }
    };
}


fn load_complete(result: Result<(), Error>, duration: Duration) -> Result<(), Error> {
    // Optionally notify another service that the LangServer is alive and serving requests
    if let Some(notifier) = get_status_notifier() {
        let message = match result {
            Ok(_) => StatusMessage::success(duration),
            Err(err) => StatusMessage::failure(err, duration),
        };
        notifier.notify(message, None)?;
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
