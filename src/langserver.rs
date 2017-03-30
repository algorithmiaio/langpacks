use base64;
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::header::{Headers, ContentType, Encoding, ContentEncoding};
use hyper::server::{Handler, Request, Response};
use hyper::status::StatusCode;
use hyper::{Get, Post, Delete};
use serde_json::{de, ser};
use serde_json::Value;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{process, thread};

use super::error::Error;
use super::langrunner::LangRunner;
use super::message::StatusMessage;
use super::notifier::Notifier;

macro_rules! jsonres {
    ($x:expr) => (concat!(r#"{"result":""#, $x, r#""}"#).to_owned());
    ($x:expr, $($arg:tt)*) => (format!(concat!(r#"{{"result":""#, $x, r#""}}"#), $($arg)*));
}
macro_rules! jsonerr {
    ($x:expr) => (concat!(r#"{"error":{"type":"SystemError","message":""#, $x, r#""}}"#).to_owned());
    ($x:expr, $($arg:tt)*) => (format!(concat!(r#"{{"error":{{"type":"SystemError","message":""#, $x, r#""}}}}"#), $($arg)*));
}

const LOG_IDENTIFIER: &'static str = "LANGSERVER";

pub enum LangServerMode {
    Sync,
    Async(Notifier),
}

pub struct LangServer {
    runner: Arc<Mutex<LangRunner>>,
    mode: LangServerMode,
    delete_signalled: Arc<Mutex<bool>>,
}

// Simple string header
header! {
    (XRequestId, "X-Request-Id") => [String]
}

impl LangServer {
    pub fn start(mode: LangServerMode, notify_exited: Option<Notifier>) -> Result<LangServer, Error> {
        let runner = LangRunner::start()?;

        let ls = LangServer {
            runner: Arc::new(Mutex::new(runner)),
            mode: mode,
            delete_signalled: Arc::new(Mutex::new(false)),
        };

        ls.monitor_runner(notify_exited);
        Ok(ls)
    }

    // Monitor runner - exit if exit is encountered
    // Since this needs a lock on the runner, it won't run while we're calling the algorithm
    fn monitor_runner(&self, notify_exited: Option<Notifier>) {
        let is_async = match self.mode {
            LangServerMode::Sync => false,
            LangServerMode::Async(..) => true,
        };
        let watched_runner = self.runner.clone();
        let watched_delete_signal = self.delete_signalled.clone();
        thread::spawn(move || {
            loop {
                let status = {
                    let r = watched_runner.lock().expect("Failed to lock runner");
                    r.check_exited()
                };

                let delete_signalled = watched_delete_signal.lock().unwrap();
                if !*delete_signalled {
                    if let Some(code) = status {
                        info!("{} {} LangServer monitor thread detected exit: {}", LOG_IDENTIFIER, "-", code);
                        if let Some(ref notifier) = notify_exited {
                            let err = Error::UnexpectedExit(code);
                            let message = StatusMessage::failure(err, Duration::new(0,0));
                            let _ = notifier.notify(message, None);
                        }
                        if !is_async {
                            process::exit(code);
                        } else {
                            break;
                        }
                    }
                } else {
                    info!("{} {} Not sending status update on delete due to explicit delete", LOG_IDENTIFIER, "-");
                    break;
                }

                thread::sleep(Duration::from_millis(500));
            }
        });
    }

    fn build_input(&self, mut req: Request, request_id: &str) -> Result<Value, Error> {
        let headers = req.headers.clone();
        let mut has_base64_content_encoding = false;
        if let Some(content_encoding_header) = headers.get::<ContentEncoding>() {
            if content_encoding_header.len() != 1 {
                return Err(Error::BadRequest("Too many ContentEncoding headers Error".to_string()));
            }

            match content_encoding_header[0] {
                Encoding::EncodingExt(ref encoding) if encoding == "base64" => {
                    has_base64_content_encoding = true;
                }
                ref encoding => return Err(Error::BadRequest(format!("Unexpected ContentEncoding {}",
                                                            encoding))),
            }
        }

        match headers.get() {
            // "application/json"
            Some(&ContentType(Mime(TopLevel::Application, SubLevel::Json, _))) => {
                info!("{} {} Handling JSON input", LOG_IDENTIFIER, request_id);
                let raw: Value = de::from_reader(req)?;
                Ok(json!({
                    "content_type": "json",
                    "data": raw,
                }))
            }
            // "text/plain"
            Some(&ContentType(Mime(TopLevel::Text, SubLevel::Plain, _))) => {
                info!("{} {} Handling text input", LOG_IDENTIFIER, request_id);
                let mut raw = String::new();
                let _ = req.read_to_string(&mut raw)?;
                Ok(json!({
                    "content_type": "text",
                    "data": raw,
                }))
            }
            // "application/octet-stream"
            Some(&ContentType(Mime(TopLevel::Application, SubLevel::Ext(_), _))) => {
                // TODO: verify sublevel is actually "octet-stream"
                info!("{} {} Handling binary input", LOG_IDENTIFIER, request_id);
                let mut raw = vec![];
                let _ = req.read_to_end(&mut raw)?;

                let result_string = if has_base64_content_encoding {
                    String::from_utf8(raw).expect("Failed to stringify bytes")
                } else {
                    base64::encode(&raw)
                };

                Ok(json!({
                    "content_type": "binary",
                    "data": result_string,
                }))
            }
            _ => Err(Error::BadRequest("Missing ContentType".to_string())),
        }
    }

    fn get_proxied_headers(&self, headers: &Headers) -> Headers {
        headers.iter()
               .filter(|h| h.name().starts_with("X-"))
               .collect()
    }

    // Returns status, response string, and a boolean to indicate if the server should terminate
    fn run_algorithm(&self, req: Request) -> (StatusCode, String, bool) {
        let headers = self.get_proxied_headers(&req.headers);
        let request_id = match req.headers.get::<XRequestId>() {
            Some(request_id) => request_id.0.to_owned(),
            None => "-".to_owned(),
        };
        let input_value = match self.build_input(req, &request_id) {
            Ok(v) => v,
            Err(err) => {
                return (StatusCode::BadRequest,
                        jsonerr!("Failed to build algorithm input from request: {}", err),
                        false);
            }
        };

        // Start piping data
        let arc_runner = self.runner.clone();
        let mut runner = arc_runner.lock().expect("Failed to take lock on runner");
        runner.set_request_id(Some(request_id.clone()));
        if let Err(err) = runner.write(&input_value) {
            error!("{} {} Failed write to runner stdin: {}", LOG_IDENTIFIER, "-", err);
            return (StatusCode::BadRequest,
                    jsonerr!("Failed to write to runner stdin: {}", err),
                    false);
        }

        // Wait for the algorithm to complete (either synchronously or asynchronously)
        match self.mode {
            LangServerMode::Sync => {
                info!("{} {} Waiting synchronously for algorithm to complete", LOG_IDENTIFIER, request_id);
                let message = runner.wait_for_response_or_exit();
                info!("{} {} algorithm completed", LOG_IDENTIFIER, request_id);
                let terminate = message.exited_early();

                match ser::to_string(&message) {
                    Ok(response) => (StatusCode::Ok, response, terminate),
                    Err(err) => (StatusCode::InternalServerError,
                                 jsonerr!("Failed to encode RunnerState: {}", err),
                                 true),
                }
            }
            LangServerMode::Async(ref notif) => {
                info!("{} {} Waiting asynchronously for algorithm to complete", LOG_IDENTIFIER, request_id);

                let notifier = notif.clone();
                let arc_runner = self.runner.clone();
                thread::spawn(move || {
                    let mut runner = arc_runner.lock().expect("Failed to take lock on runner");
                    let output  = runner.wait_for_response_or_exit();

                    let mut terminate = false;
                    if let Err(err) = notifier.notify(output, Some(headers)) {
                        error!("{} {} Failed to send REQUEST_COMPLETE notification: {}", LOG_IDENTIFIER, "-", err);
                        terminate = true;
                    }
                    if terminate {
                        let code = runner.stop();
                        process::exit(code);
                    }
                });
                info!("{} {} algorithm completed", LOG_IDENTIFIER, request_id);
                (StatusCode::Accepted, jsonres!("Algorithm started."), false)
            }
        }
    }

    fn terminate(&self) -> i32 {
        let arc_runner = self.runner.clone();
        let mut runner = arc_runner.lock().expect("Failed to take lock on runner");
        runner.stop()
    }
}



impl Handler for LangServer {
    fn handle(&self, req: Request, mut res: Response) {
        let route = format!("{} {}", req.method, req.uri);
        info!("{} {} {} (start)", LOG_IDENTIFIER, "-", route);
        let mut terminate = false;


        let (status, output) = match req.method {
            // Route for checking that LangServer is alive
            Get => (StatusCode::Ok, jsonres!("LangServer alive.")),

            // Route for calling the managed algorithm
            Post => {
                let (code, res, term) = self.run_algorithm(req);
                // Reset request id
                let arc_runner = self.runner.clone();
                let mut runner = arc_runner.lock().expect("Failed to take lock on runner");
                runner.set_request_id(None);

                terminate = term;
                (code, res)
            }

            // Route for terminating the managed algorithm
            Delete => {
                let delete_signalled = self.delete_signalled.clone();
                let mut signalled = delete_signalled.lock().unwrap();
                *signalled = true;
                let code = self.terminate();
                terminate = true;
                (StatusCode::Ok, (jsonres!("Runner exited: {}", code)))
            }

            // All other routes
            _ => (StatusCode::MethodNotAllowed, jsonerr!("Method not allowed")),
        };

        info!("{} {} {} (complete)", LOG_IDENTIFIER, "-", route);
        res.headers_mut().set(ContentType::json());
        *res.status_mut() = status;
        res.send(output.as_bytes()).unwrap();

        if terminate {
            process::exit(0);
        }
    }
}
