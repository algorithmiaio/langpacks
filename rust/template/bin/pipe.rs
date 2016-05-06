/*
* This file provides interop with the langserver
*/
extern crate algorithm;  // src/lib.rs builds into the 'algorithm' crate
extern crate algorithmia;
extern crate rustc_serialize;

use algorithmia::algo::*;
use algorithmia::error::ApiError;
use rustc_serialize::json::{self, Json};
use rustc_serialize::base64::{self, FromBase64, ToBase64};
use std::borrow::Cow;
use std::io::{self, BufRead, Write};
use std::error::Error as StdError;
use std::fs::OpenOptions;
use std::process;

const ALGOOUT: &'static str = "/tmp/algoout";

struct Request<'a> {
    data: &'a Json,
    content_type: &'a str,
}

#[derive(RustcEncodable)]
struct AlgoSuccess {
    result: Json,
    metadata: RunnerMetadata,
}

#[derive(RustcEncodable)]
struct AlgoFailure {
    error: RunnerError,
}

#[derive(RustcEncodable)]
struct RunnerMetadata {
    content_type: String,
}

#[derive(RustcEncodable)]
struct RunnerError {
    message: String,
    error_type: &'static str,
}

impl<'a> Request<'a> {
    fn from_json(json: &'a Json) -> Result<Request<'a>, String> {
        let data = json.find("data").expect("Request did not specify data field");
        let content_type = json.find("content_type")
                               .expect("Request did not specify content_type")
                               .as_string()
                               .expect("Request content_type is not a string");
        Ok(Request {
            data: data,
            content_type: content_type,
        })
    }
}

impl AlgoSuccess {
    fn new<S: Into<String>>(result: Json, content_type: S) -> AlgoSuccess {
        AlgoSuccess {
            result: result,
            metadata: RunnerMetadata { content_type: content_type.into() },
        }
    }
}

impl AlgoFailure {
    fn new<S: Into<String>>(message: S, error_type: &'static str) -> AlgoFailure {
        AlgoFailure {
            error: RunnerError {
                message: message.into(),
                error_type: error_type,
            },
        }
    }
}

fn main() {
    let algo = algorithm::Algo::default();
    println!("PIPE_INIT_COMPLETE");
    flush_std_pipes();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let output_json = match line {
            Ok(input) => {
                let output = call_algorithm(&algo, input);
                flush_std_pipes();
                serialize_output(output)
            }
            Err(err) => {
                json::encode(&AlgoFailure::new(format!("STDIN error: {}", err.description()),
                                               "SystemError"))
                    .expect("Failed to encode JSON")
            }
        };
        algoout(&output_json);
    }
}

fn serialize_output(output: Result<AlgoOutput, algorithmia::error::Error>) -> String {
    let json_result = match output {
        Ok(AlgoOutput::Text(text)) => {
            json::encode(&AlgoSuccess::new(Json::String(text), "text"))
        }
        Ok(AlgoOutput::Json(json_obj)) => {
            json::encode(&AlgoSuccess::new(json_obj, "json"))
        }
        Ok(AlgoOutput::Binary(bytes)) => {
            let config = base64::Config {
                char_set: base64::CharacterSet::Standard,
                newline: base64::Newline::LF,
                pad: true,
                line_length: None,
            };
            let result = bytes.to_base64(config);
            json::encode(&AlgoSuccess::new(Json::String(result), "binary"))
        }
        Err(err) => {
            json::encode(&AlgoFailure::new(err.description(), "AlgorithmError"))
        }
    };
    json_result.expect("Failed to encode JSON")
}

fn flush_std_pipes() {
    let _ = io::stdout().flush();
    let _ = io::stderr().flush();
}

fn algoout(output_json: &str) {
    match OpenOptions::new().write(true).open(ALGOOUT) {
        Ok(mut f) => {
            let _ = f.write(output_json.as_bytes());
            let _ = f.write(b"\n");
        }
        Err(e) => {
            println!("Cannot write to algoout pipe: {}\n", e);
            process::exit(-1);
        }
    };
}

fn call_algorithm<E: EntryPoint>(algo: &E, stdin: String) -> std::result::Result<AlgoOutput, algorithmia::error::Error> {
    let parsed = Json::from_str(&stdin).expect("Request is not valid JSON");
    let req = Request::from_json(&parsed).expect("Failed to deserialize JSON request");
    let Request { data, content_type } = req;
    let input = match (content_type, data) {
        ("text", &Json::String(ref text)) => AlgoInput::Text(Cow::Borrowed(text)),
        ("binary", &Json::String(ref encoded)) => AlgoInput::Binary(Cow::Owned(try!(encoded.from_base64()))),
        ("json", json_obj) => AlgoInput::Json(Cow::Borrowed(json_obj)),
        (ct, _) => panic!("Unsupported input content_type: {}", ct),
    };
    algo.apply(input).map_err(|e| {
        ApiError {
            message: e.description().into(),
            stacktrace: None.into(),
        }
        .into()
    })
}
