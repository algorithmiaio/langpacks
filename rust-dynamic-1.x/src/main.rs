extern crate libloading;
extern crate algorithmia;
extern crate rustc_serialize;

use algorithmia::algo::*;
use algorithmia::error::ApiError;
use libloading::*;
use rustc_serialize::{json, Decodable, Encodable};
use rustc_serialize::json::Json;
use rustc_serialize::base64::FromBase64;
use std::io::{self, BufRead, Read};

// TODO: borrow this concept for creating algooutln! and algoerrln! macros
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

#[warn(dead_code)]
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
                "text" => dl_apply(AlgoInput::Text(&body.data)).into(),
                "binary" => dl_apply(AlgoInput::Binary(&try!(body.data.from_base64()))).into(),
                "json" => dl_apply(AlgoInput::Json(body.data.clone())).into(), // TODO: wish I could kill this clone
                ct => Err(api_error(&*format!("Unsupported content_type: {}", ct))),
            }
        },
        _ =>  Err(api_error("Rust algorithms don't currently support multipart")),
      }
}


// Unfortunately, this approach is prone to segfaulting (or other unexpected behavior)
//   if the input doesn't match an algorithm signature
//   This could work if we injected a stub into the algorithm to be sure of a particular apply method,
//   but at that point, we could just inject the runner and get a bit more safety
fn dl_apply(input: AlgoInput) -> std::result::Result<AlgoResult, algorithmia::error::Error> {
    match Library::new("/home/anowell/algo/langpacks/rust-1.x/examples/hello/target/debug/libalgorithm.so") {
        Ok(lib) => match input {
            AlgoInput::Text(text) => {
                let pipe: Symbol<extern fn(&str) -> std::result::Result<AlgoResult, String>> = unsafe {
                    match lib.get(b"apply\0") {
                        Ok(sym) => sym,
                        Err(err) => return Err(api_error(&format!("Text symbol error: {}", err))),
                    }
                };
                pipe(text).map_err(|msg| api_error(&msg))

            },
            AlgoInput::Json(json) => {
                // let pipe: Symbol<extern fn(Json) -> std::result::Result<AlgoResult, String>> = unsafe {
                //     match lib.get(b"apply\0") {
                //         Ok(sym) => sym,
                //         Err(err) => panic!("Symbol error: {}", err),
                //     }
                // };
                // pipe(json).map_err(|msg| api_error(msg))
                Err(api_error("JSON input is not yet supported... more serialization work to go."))
            },
            AlgoInput::Binary(bytes) => {
                let pipe: Symbol<extern fn(&[u8]) -> std::result::Result<AlgoResult, String>> = unsafe {
                    match lib.get(b"apply\0") {
                        Ok(sym) => sym,
                        Err(err) => return Err(api_error(&format!("Binary symbol error: {}", err))),
                    }
                };
                pipe(bytes).map_err(|msg| api_error(&msg))
            }
        },
        Err(err) => panic!("Load error: {}", err),
    }
}

