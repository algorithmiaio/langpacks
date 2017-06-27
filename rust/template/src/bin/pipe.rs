// This file provides interop with the langserver
//
extern crate algorithm;
extern crate algorithmia;
extern crate base64;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use algorithmia::algo::{AlgoInput, AlgoOutput, EntryPoint};
use algorithmia::error::{Error, ErrorKind, ResultExt};
use serde_json::Value;
use std::error::Error as StdError;
use std::borrow::Cow;
use std::io::{self, BufRead, Write};
use std::fs::OpenOptions;
use std::process;

const ALGOOUT: &'static str = "/tmp/algoout";

#[derive(Deserialize)]
struct Request {
    data: Value,
    content_type: String,
}

#[derive(Serialize)]
struct AlgoSuccess {
    result: Value,
    metadata: RunnerMetadata,
}

#[derive(Serialize)]
struct AlgoFailure {
    error: RunnerError,
}

#[derive(Serialize)]
struct RunnerMetadata {
    content_type: String,
}

#[derive(Serialize)]
struct RunnerError {
    message: String,
    error_type: &'static str,
}

impl AlgoSuccess {
    fn new<S: Into<String>>(result: Value, content_type: S) -> AlgoSuccess {
        AlgoSuccess {
            result: result,
            metadata: RunnerMetadata { content_type: content_type.into() },
        }
    }
}

impl AlgoFailure {
    fn new(err: &StdError) -> AlgoFailure {
        AlgoFailure {
            error: RunnerError {
                message: error_cause_chain(err),
                error_type: "AlgorithmError",
            },
        }
    }

    fn system(err: &StdError) -> AlgoFailure {
        AlgoFailure {
            error: RunnerError {
                message: error_cause_chain(err),
                error_type: "SystemError",
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
            Err(_) => {
                let err = line.chain_err(|| "failed to read stdin").unwrap_err();
                serde_json::to_string(&AlgoFailure::system(&err as &StdError))
                    .expect("Failed to encode JSON")
            }
        };
        algoout(&output_json);
    }
}

impl From<AlgoOutput> for AlgoSuccess {
    fn from(output: AlgoOutput) -> AlgoSuccess {
        match output {
            AlgoOutput::Text(text) => AlgoSuccess::new(Value::String(text), "text"),
            AlgoOutput::Json(json_obj) => AlgoSuccess::new(json_obj, "json"),
            AlgoOutput::Binary(bytes) => {
                let result = base64::encode(&bytes);
                AlgoSuccess::new(Value::String(result), "binary")
            }
        }
    }
}

fn error_cause_chain(err: &StdError) -> String {
    let mut causes = vec![err.to_string()];
    let mut e = err;
    while let Some(cause) = e.cause() {
        causes.push(cause.to_string());
        e = cause;
    }
    causes.join("\ncaused by: ")
}

fn serialize_output(output: Result<AlgoOutput, Box<StdError>>) -> String {
    let json_result = match output {
        Ok(output) => serde_json::to_string(&AlgoSuccess::from(output)),
        Err(err) => serde_json::to_string(&AlgoFailure::new(&*err as &StdError)),
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

fn call_algorithm<E: EntryPoint>(algo: &E, stdin: String) -> Result<AlgoOutput, Box<StdError>> {
    let req = serde_json::from_str(&stdin)
        .chain_err(|| ErrorKind::DecodeJson("request"))?;
    let Request { data, content_type } = req;
    let input = match (&*content_type, data) {
        ("text", Value::String(text)) => AlgoInput::Text(Cow::Owned(text)),
        ("binary", Value::String(ref encoded)) => {
            let bytes = base64::decode(encoded)
                .chain_err(|| ErrorKind::DecodeBase64("request input"))?;
            AlgoInput::Binary(Cow::Owned(bytes))
        }
        ("json", json_obj) => AlgoInput::Json(Cow::Owned(json_obj)),
        (_, _) => return Err(
            Error::from(ErrorKind::InvalidContentType(content_type)).into(),
        ),
    };
    algo.apply(input)
}
