use base64;
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::header::{Headers, ContentType, Encoding, ContentEncoding};
use hyper::server::{Handler, Request, Response};
use hyper::status::StatusCode;
use hyper::{Get, Post, Delete};
use serde_json::{de, ser};
use serde_json::Value;
use serde_json::builder::ObjectBuilder;
use std::error::Error as StdError;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{process, thread};

use super::error::Error;
use super::langrunner::LangRunner;
use super::notifier::Notifier;

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
    pub fn new(mode: LangServerMode) -> LangServer {
        let runner = LangRunner::start().expect("Failed to start LangRunner");

        let ls = LangServer {
            runner: Arc::new(Mutex::new(runner)),
            mode: mode,
        };

        ls.monitor_runner();
        ls
    }


    fn monitor_runner(&self) {
        let watched_runner = self.runner.clone();
        thread::spawn(move || {
            loop {
                let status = {
                    let r = watched_runner.lock().expect("Failed to lock runner");
                    r.check_exited()
                };

                // Sleep even if exited, in case this exit was being handled by another thread (e.g. terminate)
                thread::sleep(Duration::from_millis(500));
                if let Some(code) = status {
                    process::exit(code);
                }
            }
        });
    }

    fn build_input(&self, mut req: Request) -> Result<Value, String> {
        let mut mutable_headers: Headers = req.headers.clone();
        let mut has_base64_content_encoding = false;
        if mutable_headers.has::<ContentEncoding>() {
            let content_encoding_header: &ContentEncoding = mutable_headers.get::<ContentEncoding>().expect("its there");
            if content_encoding_header.len() != 1 {
                return Err(jsonerr!("Too many ContentEncoding headers Error"));
            }

            match content_encoding_header.iter().next() {
                Some(&Encoding::EncodingExt(ref encoding @ _)) => {
                    if encoding == "base64" {
                        has_base64_content_encoding = true;
                    } else {
                        return Err(jsonerr!("Unexpected ContentEncoding Error"));
                    }
                }
                _ => return Err(jsonerr!("Multiple ContentEncoding Error")),
            }
        }

        if has_base64_content_encoding {
            mutable_headers.remove::<ContentEncoding>();
        }

        match mutable_headers.get() {
            // "application/json"
            Some(&ContentType(Mime(TopLevel::Application, SubLevel::Json, _))) => {
                println!("Handling JSON input");
                let raw: Value = de::from_reader(req).expect("Failed to deserialize JSON request");
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
                    String::from_utf8(b64_bytes)
                                         .expect("Failed to create string from base64 bytes")
                };

                Ok(ObjectBuilder::new()
                       .insert("content_type", "binary")
                       .insert("data", Value::String(result_string))
                       .unwrap())
            }
            _ => Err(jsonerr!("Missing ContentType")),
        }
    }

    fn get_proxied_headers(&self, headers: &Headers) -> Headers {
        headers.iter()
               .filter(|h| h.name().starts_with("X-"))
               .collect()
    }

    fn run_algorithm(&self, req: Request) -> Result<Option<String>, Error> {
        let headers = self.get_proxied_headers(&req.headers);
        let input_value = self.build_input(req)
                              .expect("Failed to build algorithm input from request");

        // Start piping data
        let arc_runner = self.runner.clone();
        let mut runner = arc_runner.lock().expect("Failed to take lock on runner");
        if let Err(err) = runner.write(&input_value) {
            println!("Failed write to runner stdin: {}", err);
            return Err(err);
        }

        // Wait for the algorithm to complete (either synchronously or asynchronously)
        match self.mode {
            LangServerMode::Sync => {
                println!("Waiting synchronously for algorithm to complete");
                let runner_output = try!(runner.wait_for_response());
                let response = try!(ser::to_string(&runner_output));
                Ok(Some(response))
            }
            LangServerMode::Async(ref notif) => {
                println!("Waiting asynchronously for algorithm to complete");

                let notifier = notif.clone();
                let arc_runner = self.runner.clone();
                thread::spawn(move || {
                    let mut runner = arc_runner.lock().expect("Failed to take lock on runner");
                    let response = runner.wait_for_response().expect("Failed waiting for response");

                    if let Err(err) = notifier.notify(response, Some(headers)) {
                        println!("Failed to send REQUEST_COMPLETE notification: {}", err);
                        let code = runner.stop();
                        process::exit(code);
                    }
                });
                Ok(None)
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
                match self.run_algorithm(req) {
                    Ok(Some(out)) => (StatusCode::Ok, out),
                    Ok(None) => (StatusCode::Accepted, jsonres!("Algorithm started.")),
                    Err(err) => {
                        println!("Request Failed: {}", err);
                        match err.cause() {
                            Some(cause) => (StatusCode::BadRequest,
                                            jsonerr!("{} - {}", err.description(), cause)),
                            None => (StatusCode::BadRequest, err.description().to_owned()),
                        }
                    }
                }
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
