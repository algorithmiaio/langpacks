use serde::ser::Serialize;
use serde_json::ser;
use serde_json::de::StreamDeserializer;
use serde_json::Value;
use std::collections::BTreeMap;
use std::env;
use std::io::{Read, Write, BufRead, BufReader};
use std::fs::File;
use std::process::{Command, Child, ChildStdin, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use time::{self, PreciseTime};
use wait_timeout::ChildExt;

use super::error::Error;

const ALGOOUT: &'static str = "/tmp/algoout";

struct RunnerOutput(Value);

pub struct LangRunner {
    child_stdout: Arc<Mutex<Vec<String>>>,
    child_stdin: Mutex<Option<ChildStdin>>,
    child: Mutex<Child>,
}

impl LangRunner {
    pub fn start() -> Result<LangRunner, Error> {
        let mut path = try!(env::current_dir());
        path.push("bin/pipe");

        let mut child = try!(Command::new(&path)
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .stderr(Stdio::null())
                            .spawn());

        println!("Running PID {}: {}", child.id(), path.to_string_lossy());

        let stdin = try!(child.stdin.take()
            .ok_or(Error::Unexpected(s!("Failed to open runner's STDIN")))
        );
        let stdout = try!(child.stdout.take()
            .ok_or(Error::Unexpected(s!("Failed to open runner's STDOUT")))
        );

        let child_stdout = Arc::new(Mutex::new(Vec::new()));

        let arc_stdout = child_stdout.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line_result in reader.lines() {
                match line_result {
                    Ok(line) => match arc_stdout.lock() {
                        Ok(mut lines) => lines.push(line),
                        Err(err) => println!("Failed to get lock on stdout lines: {}", err),
                    },
                    Err(err) => println!("Failed to read line: {}", err),
                }
            }

        });

        Ok(LangRunner {
            child: Mutex::new(child),
            child_stdin: Mutex::new(Some(stdin)),
            child_stdout: child_stdout,
        })
    }


    pub fn write<T: Serialize>(&self, input: &T) -> Result<(), Error> {
        let mut stdin_lock = self.child_stdin.lock().expect("Failed to get stdin lock");
        match stdin_lock.as_mut() {
            Some(mut stdin) => {
                println!("Sending data to runner stdin");
                try!(ser::to_writer(&mut stdin, &input));
                try!(stdin.write(b"\n"));
                Ok(())
            }
            None => {
                Err(Error::Unexpected("cannot write to closed runner stdin".to_owned()))
            }
        }
    }

    pub fn wait_for_response(&self) -> Result<Value, Error> {
        println!("Opening /tmp/algoout FIFO...");
        let start = PreciseTime::now();

        // Note: Opening a FIFO read-only pipe blocks until a writer opens it. Would be nice to open with O_NONBLOCK
        let algoout = try!(File::open(ALGOOUT));

        // Collect runner output from JSON stream - reads and deserializes the single next JSON Value on algout
        println!("Deserializing algoout stream...");
        let mut algoout_stream: StreamDeserializer<Value, _> =
            StreamDeserializer::new(algoout.bytes());

        // try to read next json value, then try to deserialize
        let output = match algoout_stream.next() {
            Some(next) => match next {
                Ok(out) => out,
                Err(err) => {
                    println!("Failed to deserialize next JSON value from stream: {}", err);
                    return Err(err.into());
                }
            },
            None => {
                return Err(Error::Unexpected("No more JSON to read from the stream".to_owned()));
            }
        };

        let duration = start.to(PreciseTime::now());

        // Collect buffered stdout - grab lock on child_stdout, and join all the buffered lines
        let mut algo_stdout;
        let arc_stdout = self.child_stdout.clone();
        {
            let mut lines = arc_stdout.lock().expect("Failed to get lock on stdout lines");
            algo_stdout = lines.join("\n");
            let _ = algo_stdout.pop();
            lines.clear();
        }

        // Augment output with duration and stdout
        let mut runner_output = RunnerOutput(output);
        runner_output.set_duration(duration);
        runner_output.set_stdout(algo_stdout);

        Ok(runner_output.0)
    }


    pub fn wait_for_exit(&self) -> Option<i32> {
        {
            // Mutably `take` child_stdin out of `self` and then let it go out of scope, resulting in EOF
            let mut stdin_lock = self.child_stdin.lock().expect("Failed to take stdin lock");
            let _drop_stdin = stdin_lock.take();
            println!("Dropping runner stdin.");
        }

        // Now that stdin is closed, we can wait on child
        println!("Waiting for runner to exit...");
        let mut child = self.child.lock().expect("Failed to get lock on runner");
        let status = child.wait_timeout(Duration::from_secs(3)).expect("Failed to wait on runner");
        match status {
            Some(exit) => {
                println!("Runner exited: {:?}", &exit);
                exit.code()
            }
            None => {
                println!("Runner did not exit. Killing.");
                if let Err(err) = child.kill() {
                    println!("Failed to kill runner: {}", err);
                }
                None
            }
        }
    }
}

impl RunnerOutput {
    fn set_duration(&mut self, duration: time::Duration) {
        let duration_micro = duration.num_microseconds().unwrap() as f64 / 1_000_000f64;
        let mut metadata = self.metadata_mut();
        metadata.insert(s!("duration"), Value::F64(duration_micro));
    }

    fn set_stdout(&mut self, stdout: String) {
        if !stdout.is_empty() {
            let mut metadata = self.metadata_mut();
            metadata.insert(s!("stdout"), Value::String(stdout));
        }
    }

    fn metadata_mut(&mut self) -> &mut BTreeMap<String, Value> {
        let mut metadata = match self.0.as_object_mut() {
            Some(map) => {
                match map.contains_key("metadata") {
                    true => map.get_mut("metadata").unwrap(),
                    false => {
                        let metadata = BTreeMap::new();
                        map.insert(s!("metadata"), Value::Object(metadata));
                        map.get_mut("metadata").expect("Failed to insert and retrieve metadata")
                    }
                }
            }
            None => panic!("Output not a valid structure"),
        };
        metadata.as_object_mut().expect("metadata is not an object")
    }
}
