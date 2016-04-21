use base64;
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::header::{Headers, ContentType, Encoding, ContentEncoding};
use hyper::server::{Handler, Request, Response};
use hyper::status::StatusCode;
use hyper::{Get, Post, Delete};
use serde_json::{de, ser};
use serde_json::Value;
use serde_json::builder::ObjectBuilder;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{process, thread};

use super::error::Error;
use super::langrunner::LangRunner;
use super::message::{RunnerOutput, StatusMessage};
use super::notifier::{Notifier, HealthStatus};

macro_rules! jsonres {
    ($x:expr) => (concat!(r#"{"result":""#, $x, r#""}"#).to_owned());
    ($x:expr, $($arg:tt)*) => (format!(concat!(r#"{{"result":""#, $x, r#""}}"#), $($arg)*));
}
macro_rules! jsonerr {
    ($x:expr) => (concat!(r#"{"error":{"type":"SystemError","message":""#, $x, r#""}}"#).to_owned());
    ($x:expr, $($arg:tt)*) => (format!(concat!(r#"{{"error":{{"type":"SystemError","message":""#, $x, r#""}}}}"#), $($arg)*));
}

pub enum LangServerMode {
    Sync,
    Async(Notifier),
}

pub struct LangServer {
    runner: Arc<Mutex<LangRunner>>,
    mode: LangServerMode,
}

impl LangServer {
    pub fn new(mode: LangServerMode, notify_exited: Option<Notifier>) -> LangServer {
        let runner = LangRunner::start().expect("Failed to start LangRunner");

        let ls = LangServer {
            runner: Arc::new(Mutex::new(runner)),
            mode: mode,
        };

        ls.monitor_runner(notify_exited);
        ls
    }

    // Monitor runner - exit if exit is encountered
    // Since this needs a lock on the runner, it won't run while we're calling the algorithm
    fn monitor_runner(&self, notify_exited: Option<Notifier>) {
        let watched_runner = self.runner.clone();
        thread::spawn(move || {
            loop {
                let status = {
                    let r = watched_runner.lock().expect("Failed to lock runner");
                    r.check_exited()
                };

                if let Some(code) = status {
                    println!("LangServer monitor thread detected exit: {}", code);
                    if let Some(notifier) = notify_exited {
                        let health_status = HealthStatus::Failure(Error::UnexpectedExit(code));
                        let r = watched_runner.lock().expect("Failed to lock runner");
                        let (stdout, stderr) = r.consume_stdio();
                        let message = StatusMessage::new(health_status, Duration::new(0,0), Some(stdout), Some(stderr));
                        let _ = notifier.notify(message, None);
                    }
                    process::exit(code);
                }

                thread::sleep(Duration::from_millis(500));
            }
        });
    }

    fn build_input(&self, mut req: Request) -> Result<Value, Error> {
        let mut mutable_headers: Headers = req.headers.clone();
        let mut has_base64_content_encoding = false;
        if mutable_headers.has::<ContentEncoding>() {
            let content_encoding_header: &ContentEncoding =
                mutable_headers.get::<ContentEncoding>().expect("its there");
            if content_encoding_header.len() != 1 {
                return Err(Error::BadRequest("Too many ContentEncoding headers Error".to_string()));
            }

            match content_encoding_header.iter().next() {
                Some(&Encoding::EncodingExt(ref encoding @ _)) => {
                    if encoding == "base64" {
                        has_base64_content_encoding = true;
                    } else {
                        return Err(Error::BadRequest(format!("Unexpected ContentEncoding {}",
                                                             encoding)));
                    }
                }
                _ => return Err(Error::BadRequest("Multiple ContentEncoding Error".to_string())),
            }
        }

        if has_base64_content_encoding {
            mutable_headers.remove::<ContentEncoding>();
        }

        match mutable_headers.get() {
            // "application/json"
            Some(&ContentType(Mime(TopLevel::Application, SubLevel::Json, _))) => {
                println!("Handling JSON input");
                let raw: Value = try!(de::from_reader(req));
                Ok(ObjectBuilder::new()
                       .insert("content_type", "json")
                       .insert("data", raw)
                       .unwrap())
            }
            // "text/plain"
            Some(&ContentType(Mime(TopLevel::Text, SubLevel::Plain, _))) => {
                println!("Handling text input");
                let mut raw = String::new();
                let _ = req.read_to_string(&mut raw).expect("Failed to read request");
                Ok(ObjectBuilder::new()
                       .insert("content_type", "text")
                       .insert("data", Value::String(raw))
                       .unwrap())
            }
            // "application/octet-stream"
            Some(&ContentType(Mime(TopLevel::Application, SubLevel::Ext(_), _))) => {
                // TODO: verify sublevel is actually "octet-stream"
                println!("Handling binary input");
                let mut raw: Vec<u8> = vec![];
                let _ = req.read_to_end(&mut raw).expect("Failed to read request");

                let result_string = if has_base64_content_encoding {
                    String::from_utf8(raw).expect("Failed to stringify bytes")
                } else {
                    let b64_bytes = base64::u8en(&raw).expect("Failed encode request as base64");
                    String::from_utf8(b64_bytes).expect("Failed to create string from base64 bytes")
                };

                Ok(ObjectBuilder::new()
                       .insert("content_type", "binary")
                       .insert("data", Value::String(result_string))
                       .unwrap())
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
        let input_value = match self.build_input(req) {
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
        if let Err(err) = runner.write(&input_value) {
            println!("Failed write to runner stdin: {}", err);
            return (StatusCode::BadRequest,
                    jsonerr!("Failed to write to runner stdin: {}", err),
                    false);
        }

        // Wait for the algorithm to complete (either synchronously or asynchronously)
        match self.mode {
            LangServerMode::Sync => {
                println!("Waiting synchronously for algorithm to complete");
                let (status_code, output, terminate) = match runner.wait_for_response_or_exit() {
                    RunnerOutput::Completed(output) => (StatusCode::Ok, output, false),
                    RunnerOutput::Exited(output) => (StatusCode::Ok, output, true),
                };

                match ser::to_string(&output) {
                    Ok(response) => (status_code, response, terminate),
                    Err(err) => (StatusCode::InternalServerError,
                                 jsonerr!("Failed to encode RunnerOutput: {}", err),
                                 true),
                }
            }
            LangServerMode::Async(ref notif) => {
                println!("Waiting asynchronously for algorithm to complete");

                let notifier = notif.clone();
                let arc_runner = self.runner.clone();
                thread::spawn(move || {
                    let mut runner = arc_runner.lock().expect("Failed to take lock on runner");
                    let (output, mut terminate) = match runner.wait_for_response_or_exit() {
                        RunnerOutput::Completed(output) => (output, false),
                        RunnerOutput::Exited(output) => (output, true),
                    };

                    if let Err(err) = notifier.notify(output, Some(headers)) {
                        println!("Failed to send REQUEST_COMPLETE notification: {}", err);
                        terminate = true;
                    }
                    if terminate {
                        let code = runner.stop();
                        process::exit(code);
                    }
                });
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
        println!("{} (start)", route);
        let mut terminate = false;


        let (status, output) = match req.method {
            // Route for checking that LangServer is alive
            Get => (StatusCode::Ok, jsonres!("LangServer alive.")),

            // Route for calling the managed algorithm
            Post => {
                let (code, res, term) = self.run_algorithm(req);
                terminate = term;
                (code, res)
            }

            // Route for terminating the managed algorithm
            Delete => {
                let code = self.terminate();
                terminate = true;
                (StatusCode::Ok, (jsonres!("Runner exited: {}", code)))
            }

            // All other routes
            _ => (StatusCode::MethodNotAllowed, jsonerr!("Method not allowed")),
        };

        println!("{} (complete)", route);
        res.headers_mut().set(ContentType::json());
        *res.status_mut() = status;
        res.send(output.as_bytes()).unwrap();

        if terminate {
            process::exit(0);
        }
    }
}
