use base64;
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::header::{Headers, ContentType};
use hyper::server::{Handler, Request, Response};
use hyper::status::StatusCode;
use hyper::{Get, Post, Delete};
use serde_json::{de, ser};
use serde_json::Value;
use serde_json::builder::ObjectBuilder;
use std::error::Error as StdError;
use std::io::{Read, Write};
use std::sync::Arc;
use std::{process, thread};

use super::error::Error;
use super::langrunner::LangRunner;
use super::notifier::Notifier;

pub enum LangServerMode {
    Sync,
    Async(Notifier),
}

pub struct LangServer {
    runner: Arc<LangRunner>,
    mode: LangServerMode,
}

impl LangServer {
    pub fn new(mode: LangServerMode) -> LangServer {
        LangServer {
            runner: Arc::new(LangRunner::start().expect("Failed to start LangRunner")),
            mode: mode,
        }
    }

    fn build_input(&self, mut req: Request) -> Result<Value, String> {
        match req.headers.get() {
            // "application/json"
            Some(&ContentType(Mime(TopLevel::Application, SubLevel::Json, _))) => {
                let raw: Value = de::from_reader(req).expect("Failed to deserialize JSON request");
                Ok(ObjectBuilder::new()
                       .insert("content_type", "json")
                       .insert("data", raw)
                       .unwrap())
            }
            // "text/plain"
            Some(&ContentType(Mime(TopLevel::Text, SubLevel::Plain, _))) => {
                let mut raw = String::new();
                let _ = req.read_to_string(&mut raw).expect("Failed to read request");
                Ok(ObjectBuilder::new()
                       .insert("content_type", "json")
                       .insert("data", Value::String(raw))
                       .unwrap())
            }
            // "application/octet-stream"
            Some(&ContentType(Mime(TopLevel::Application, SubLevel::Ext(_), _))) => {
                // TODO: verify sublevel is actually "octet-stream"
                let mut raw: Vec<u8> = vec![];
                let _ = req.read_to_end(&mut raw).expect("Failed to read request");
                let b64_bytes = base64::u8en(&raw).expect("Failed encode request as base64");
                let b64_string = String::from_utf8(b64_bytes)
                                     .expect("Failed to create string from base64 bytes");
                Ok(ObjectBuilder::new()
                       .insert("content_type", "binary")
                       .insert("data", Value::String(b64_string))
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
        // TODO: freak out if another request is in progress

        let headers = self.get_proxied_headers(&req.headers);
        let input_value = self.build_input(req)
                              .expect("Failed to build algorithm input from request");

        // Start piping data
        let arc_runner = self.runner.clone();
        if let Err(err) = arc_runner.write(&input_value) {
            println!("Failed write to runner stdin: {}", err);
            return Err(err);
        }

        // Wait for the algorithm to complete (either synchronously or asynchronously)
        match self.mode {
            LangServerMode::Sync => {
                println!("Waiting synchronously for algorithm to complete");
                let runner_output = try!(arc_runner.wait_for_response());
                let response = try!(ser::to_string(&runner_output));
                Ok(Some(response))
            }
            LangServerMode::Async(ref notif) => {
                println!("Waiting asynchronously for algorithm to complete");

                let notifier = notif.clone();
                let arc_runner = self.runner.clone();
                thread::spawn(move || {
                    let response = arc_runner.wait_for_response().expect("Failed waiting for response");

                    if let Err(err) = notifier.notify(response, Some(headers)) {
                        println!("Failed to send REQUEST_COMPLETE notification: {}", err);
                    }
                });
                Ok(None)
            }
        }
    }

    fn terminate(&self) -> Option<i32> {
        let arc_runner = self.runner.clone();
        arc_runner.wait_for_exit()
    }
}

impl Handler for LangServer {

    fn handle(&self, req: Request, mut res: Response) {
        let route = format!("{} {}", req.method, req.uri);
        println!("{} (start)", route);
        let mut terminate = false;


        let (status, output) = match req.method {
            // Route for checking that LangServer is alive
            Get => (StatusCode::Ok, s!(r#""LangServer alive.""#)),

            // Route for calling the managed algorithm
            Post => {
                match self.run_algorithm(req) {
                    Ok(Some(out)) => (StatusCode::Ok, out),
                    Ok(None) => (StatusCode::Accepted, s!(r#""Algorithm started.""#)),
                    Err(err) => {
                        println!("Request Failed: {}", err);
                        match err.cause() {
                            Some(cause) => (StatusCode::BadRequest, format!("{} - {}", err.description(), cause)),
                            None => (StatusCode::BadRequest, err.description().to_owned()),
                        }
                    }
                }
            }

            // Route for terminating the managed algorithm
            Delete => {
                let code = self.terminate();
                terminate = true;
                (StatusCode::Ok, (format!("Runner exited: {:?}", code)))
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
