extern crate hyper;
extern crate serde_json;
extern crate base64;
extern crate time;

use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::header::ContentType;
use hyper::server::{Handler, Server, Request, Response};
use hyper::status::StatusCode;
use serde_json::de::StreamDeserializer;
use serde_json::Value;
use serde_json::builder::ObjectBuilder;
use std::env;
use std::io::{Read, Write, BufRead, BufReader};
use std::fs::File;
use std::process::{Command, ChildStdin, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use time::PreciseTime;

const ALGOOUT: &'static str = "/tmp/algoout";

macro_rules! s { ($x:expr) => ($x.to_string()); }
macro_rules! jsonerr {
    ($x:expr) => (concat!(r#"{"error":{"message":""#, $x, r#""}}"#).into());
}

struct LangServer {
    child_stdin: Arc<Mutex<ChildStdin>>,
    child_stdout_lines: Arc<Mutex<Vec<String>>>,
}

impl LangServer {
    fn new() -> LangServer {
        let mut path = env::current_dir().expect("Failed to get working directory");
        path.push("bin/pipe");
        println!("Running {:?}", path);
        let mut child = Command::new(path)
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn()
                                .unwrap_or_else(|e| { panic!("failed to execute child: {}", e) });

        let stdin = child.stdin.take().expect("Failed to open runner's STDIN");
        let stdout = child.stdout.take().expect("Failed to open runner's STDOUT");

        let child_stdout_lines = Arc::new(Mutex::new(Vec::new()));

        let arc_lines = child_stdout_lines.clone();
        thread::spawn(move|| {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
              let mut lines = arc_lines.lock().expect("Failed to get lock on stdout lines");
              lines.push(line.expect("Failed to read line"));
            }
        });

        LangServer{
            child_stdin: Arc::new(Mutex::new(stdin)),
            child_stdout_lines: child_stdout_lines,
        }
    }

    fn build_input(&self, mut req: Request) -> Result<Value, String> {
        match req.headers.get() {
            // "application/json"
            Some(&ContentType(Mime(TopLevel::Application, SubLevel::Json, _))) => {
                let raw: Value = serde_json::de::from_reader(req).expect("Failed to deserialize JSON request");
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
                    .insert("content_type", "text")
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

    fn run_algorithm(&self, req: Request) -> Result<String, String> {
        // TODO: freak out if another request is in progress

        let input_value = self.build_input(req).expect("Failed to build algorithm input from request");

        // Get a lock on the child stdin/stdout handle
        let arc_stdin = self.child_stdin.clone();
        let mut stdin = arc_stdin.lock().expect("Failed to get lock on runner's STDIN");

        // Start piping data
        let start = PreciseTime::now();
        serde_json::ser::to_writer(&mut *stdin, &input_value).expect("Failed to write input to runner's STDIN");
        stdin.write(b"\n").expect("Failed to write new line to runner's STDIN");

        // Opening the pipe AFTER because opening a FIFO read-only pipe blocks until a writer opens it
        let algoout = File::open(ALGOOUT).expect("Failed to open ALGOOUT pipe");

        // Collect runner output from JSON stream
        let mut algoout_stream: StreamDeserializer<Value, _> = StreamDeserializer::new(algoout.bytes());
        let mut output = algoout_stream.next().expect("Failed to read next JSON value from stream").expect("Failed to deserialize next JSON value from stream");
        let duration = start.to(PreciseTime::now());
        let duration_micro = duration.num_microseconds().unwrap() as f64 / 1_000_000f64;

        // Collect buffered stdout
        let mut algo_stdout;
        let arc_lines = self.child_stdout_lines.clone();
        {
          let mut lines = arc_lines.lock().expect("Failed to get lock on stdout lines");
          algo_stdout = lines.join("\n");
          let _ = algo_stdout.pop();
          lines.clear();
        }

        // Augment runner output
        match output.as_object_mut() {
            Some(map) => match map.get_mut("metadata") {
                Some(metadata) => {
                    let metadata_obj = metadata.as_object_mut().unwrap();
                    metadata_obj.insert(s!("duration"), Value::F64(duration_micro));
                    if !algo_stdout.is_empty() {
                        metadata_obj.insert(s!("stdout"), Value::String(algo_stdout));
                    }
                },
                None => panic!("TODO: do we nee to augment error response?"),
            },
            None => panic!("Output not a valid structure"),
        };

        let response = serde_json::ser::to_string(&output).expect("Failed to serialize respons JSON");
        Ok(response)
    }

}
impl Handler for LangServer {
    fn handle(&self, req: Request, mut res: Response) {
        let route = format!("{} {}", req.method, req.uri);
        println!("{} (start)", route);

        let (status, output) = match req.method {
            hyper::Get => (StatusCode::Ok, s!("LangPackServer alive...")),
            hyper::Post => match self.run_algorithm(req) {
                Ok(out) => (StatusCode::Ok, out),
                Err(err) => (StatusCode::BadRequest, err),
            },
            _ => (StatusCode::MethodNotAllowed, jsonerr!("Method not allowed")),
        };

        println!("{} (complete)", route);
        res.headers_mut().set(ContentType::json());
        *res.status_mut() = status;
        res.send(output.as_bytes()).unwrap();
    }
}

fn main() {
    let handler = LangServer::new();
    let listener = Server::http("0.0.0.0:3000").unwrap().handle(handler).unwrap();
    println!("Listening on port 3000.");


    // TODO: on sigterm, close listener, let listener guard fall out of scope, wait_with_timeout for child to exit
    // listener.close();
}
