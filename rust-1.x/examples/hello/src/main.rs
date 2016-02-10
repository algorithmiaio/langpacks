extern crate algorithm;
extern crate algorithmia;
extern crate rustc_serialize;

use algorithmia::algo::*;
use algorithmia::error::ApiError;
use rustc_serialize::json;
use rustc_serialize::base64::FromBase64;
use std::io::{self, BufRead};

// algoutln! is println! for the algoout named pipe
macro_rules! algooutln {
    ($fmt:expr) => ({
        use std::fs::OpenOptions;
        use std::io::Write;
        match OpenOptions::new().write(true).open("/tmp/algoout") {
            Ok(mut f) => { let _ = f.write_fmt(format_args!(concat!($fmt, "\n"))); },
            Err(e) => { let _ = ::std::io::stderr().write_fmt(format_args!("Cannot write to algoout pipe: {}\n", e)); },
        };
    });
    ($fmt:expr, $($arg:tt)*) => ({
        use std::fs::OpenOptions;
        use std::io::Write;
        match OpenOptions::new().write(true).open("/tmp/algoout") {
            Ok(mut f) => { let _ = f.write_fmt(format_args!(concat!($fmt, "\n"), $($arg)*)); },
            Err(e) => { let _ = ::std::io::stderr().write_fmt(format_args!("Cannot write to algoout pipe: {}\n", e)); },
        };
    });
}

#[derive(RustcDecodable)]
struct Request {
    body: Vec<BodyPart>
}

#[allow(dead_code)]
#[derive(RustcDecodable)]
struct BodyPart {
    name: String,
    filename: Option<String>,
    data: String, // TODO: Make the this as String or Json type of object
    content_type: String,
}

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match line {
            Ok(input) => match call_algorithm(input) {
                Ok(output) => match output {
                    AlgoResult::Void => println!("VOID"),
                    AlgoResult::Text(text) => algooutln!("text\n{}", text),
                    AlgoResult::Json(json) => println!("json: {}", json),
                    AlgoResult::Binary(_) => println!("binary..."),
                },
                Err(err) => {
                    // TODO: spit it out to algoerr
                    println!("Error: {}", err);
                },
            },
            Err(err) => println!("Stdin error: {}", err)
        }
    }

}

fn api_error(msg: &str) -> algorithmia::error::Error {
    ApiError{message: msg.into(), stacktrace: None}.into()
}

fn call_algorithm(stdin: String) -> std::result::Result<AlgoResult, algorithmia::error::Error> {
      let req = try!(json::decode::<Request>(&stdin));

      match req.body.len() {
        0 => Err(api_error("Empty Request")),
        1 => {
            let body = req.body.first().unwrap();
            match &*body.content_type {
                "text" => algorithm::apply(AlgoInput::Text(&body.data)),
                "binary" => algorithm::apply(AlgoInput::Binary(&try!(body.data.from_base64()))),
                "json" => algorithm::apply(AlgoInput::Json(body.data.clone())), // TODO: wish I could kill this clone
                ct => Err(format!("Unsupported content_type: {}", ct)),
            }.map_err(|e| api_error(&*e))
        },
        _ =>  Err(api_error("Rust algorithms don't currently support multipart")),
      }
}


