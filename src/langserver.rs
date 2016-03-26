use base64;
use hyper::client::Client;
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::header::ContentType;
use hyper::server::{Handler, Request, Response};
use hyper::status::StatusCode;
use hyper::{Url, Get, Post, Delete};
use serde_json::de;
use serde_json::Value;
use serde_json::builder::ObjectBuilder;
use std::io::{Read, Write};
use std::sync::Arc;
use std::thread;

use super::langrunner::LangRunner;

pub enum LangServerMode {
    Sync,
    Async(Url),
}

header! { (XRequestId, "X-Request-ID") => [String] }

pub struct LangServer {
    runner: Arc<LangRunner>,
    mode: Arc<LangServerMode>,
}

impl LangServer {
    pub fn new(mode: LangServerMode) -> LangServer {
        LangServer{
            runner: Arc::new(LangRunner::new()),
            mode: Arc::new(mode),
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
            },
            // "text/plain"
            Some(&ContentType(Mime(TopLevel::Text, SubLevel::Plain, _))) => {
                let mut raw = String::new();
                let _ = req.read_to_string(&mut raw).expect("Failed to read request");
                Ok(ObjectBuilder::new()
                    .insert("content_type", "json")
                    .insert("data", Value::String(raw))
                    .unwrap())
            },
            // "application/octet-stream"
            Some(&ContentType(Mime(TopLevel::Application, SubLevel::Ext(_), _))) => {
                // TODO: verify sublevel is actually "octet-stream"
                let mut raw: Vec<u8> = vec![];
                let _ = req.read_to_end(&mut raw).expect("Failed to read request");
                let b64_bytes = base64::u8en(&raw).expect("Failed encode request as base64");
                let b64_string = String::from_utf8(b64_bytes).expect("Failed to create string from base64 bytes");
                Ok(ObjectBuilder::new()
                    .insert("content_type", "binary")
                    .insert("data", Value::String(b64_string))
                    .unwrap())
            },
            _ => Err(jsonerr!("Missing ContentType")),
        }
    }

    fn run_algorithm(&self, req: Request) -> Result<Option<String>, String> {
        // TODO: freak out if another request is in progress

        let request_id = req.headers.get::<XRequestId>().map(|h| h.to_string()).unwrap_or("no-id".into());
        let input_value = self.build_input(req).expect("Failed to build algorithm input from request");

        // Start piping data
        let arc_runner = self.runner.clone();
        arc_runner.write(&input_value); // TODO: add error handling


        // Wait for the algorithm to complete (either synchronously or asynchronously)
        let mode = self.mode.clone();
        match &*mode {
            &LangServerMode::Sync => {
              println!("Waiting synchronously for algorithm to complete");
              let response = arc_runner.wait_for_response().expect("Failed waiting for response");
              Ok(Some(response))
            },
            &LangServerMode::Async(ref url) => {
              println!("Waiting asynchronously for algorithm to complete");
              let callback_url = url.clone();
              let arc_runner = self.runner.clone();
              thread::spawn( move|| {
                  let response = arc_runner.wait_for_response().expect("Failed waiting for response");
                  if let Err(err) = Client::new()
                                        .post(callback_url)
                                        .header(ContentType::json())
                                        .header(XRequestId(request_id))
                                        .body(&response)
                                        .send() {
                      println!("Failed to send notification that request completed: {}", err);
                  }
              });
              Ok(None)
            },
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

        let (status, output) = match req.method {
            Get => (StatusCode::Ok, jsonres!("LangServer alive.")),
            Post => match self.run_algorithm(req) {
                Ok(Some(out)) => (StatusCode::Ok, out),
                Ok(None) => (StatusCode::Accepted, jsonres!("Algorithm started.")),
                Err(err) => (StatusCode::BadRequest, err),
            },
            Delete => {
                let code = self.terminate();
                (StatusCode::Ok, (format!("Runner exited: {:?}", code)))
            },
            _ => (StatusCode::MethodNotAllowed, jsonerr!("Method not allowed")),
        };

        println!("{} (complete)", route);
        res.headers_mut().set(ContentType::json());
        *res.status_mut() = status;
        res.send(output.as_bytes()).unwrap();
    }
}