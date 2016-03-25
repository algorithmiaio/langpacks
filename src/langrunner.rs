use serde::ser::Serialize;
use serde_json::ser;
use serde_json::de::StreamDeserializer;
use serde_json::Value;
use std::env;
use std::io::{Read, Write, BufRead, BufReader};
use std::fs::File;
use std::process::{Command, Child, ChildStdin, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use time::PreciseTime;

const ALGOOUT: &'static str = "/tmp/algoout";


pub struct LangRunner {
    pub child_stdout: Arc<Mutex<Vec<String>>>, //TODO: Option - we often don't care about stdout
    child_stdin: Mutex<Option<ChildStdin>>,
    child: Mutex<Child>,
}

impl LangRunner {
    pub fn new() -> LangRunner {
        let mut path = env::current_dir().expect("Failed to get working directory");
        path.push("bin/pipe");
        let mut child = Command::new(&path)
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .stderr(Stdio::null())
                                .spawn()
                                .unwrap_or_else(|e| { panic!("failed to execute child: {}", e) });
        println!("Running PID {} {:?}", child.id(), path);

        let stdin = child.stdin.take().expect("Failed to open runner's STDIN");
        let stdout = child.stdout.take().expect("Failed to open runner's STDOUT");

        let child_stdout = Arc::new(Mutex::new(Vec::new()));

        let arc_stdout = child_stdout.clone();
        thread::spawn(move|| {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
              let mut lines = arc_stdout.lock().expect("Failed to get lock on stdout lines");
              lines.push(line.expect("Failed to read line"));
            }
        });

        LangRunner{
            child: Mutex::new(child),
            child_stdin: Mutex::new(Some(stdin)),
            child_stdout: child_stdout,
        }
    }


    pub fn write<T: Serialize>(&self, input: &T) {
        let mut stdin_lock = self.child_stdin.lock().expect("Failed to get stdin lock");
        match stdin_lock.as_mut() {
            Some(mut stdin) => {
                println!("Sending data to runner stdin");
                ser::to_writer(&mut stdin, &input).expect("Failed to write input to runner's STDIN");
                stdin.write(b"\n").expect("Failed to write new line to runner's STDIN");
            },
            None => {
                panic!("Child stdin has already been moved");
            }
        };
    }

    pub fn wait_for_response(&self) -> Result<String, String> {
        println!("Opening algoout FIFO...");
        let start = PreciseTime::now();

        // Note: Opening a FIFO read-only pipe blocks until a writer opens it. Would be nice to open with O_NONBLOCK
        // TODO: make this non-blocking, because otherwise this is a potential deadlock if the runner crashes before opening ALGOOUT
        let algoout = File::open(ALGOOUT).expect("Failed to open ALGOOUT pipe");

        // Collect runner output from JSON stream - reads and deserializes the single next JSON Value on algout
        println!("Deserializing algoout stream...");
        let mut algoout_stream: StreamDeserializer<Value, _> = StreamDeserializer::new(algoout.bytes());
        let mut output = algoout_stream.next().expect("Failed to read next JSON value from stream").expect("Failed to deserialize next JSON value from stream");
        let duration = start.to(PreciseTime::now());
        let duration_micro = duration.num_microseconds().unwrap() as f64 / 1_000_000f64;

        // Collect buffered stdout - grab lock on child_stdout, and join all the buffered lines
        let mut algo_stdout;
        let arc_stdout = self.child_stdout.clone();
        {
          let mut lines = arc_stdout.lock().expect("Failed to get lock on stdout lines");
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

        let response = ser::to_string(&output).expect("Failed to serialize respons JSON");
        Ok(response)
    }

    pub fn wait_for_exit(&self) -> i32 {
        {
          // Mutably `take` child_stdin out of `self` and then let it go out of scope, resulting in EOF
          let mut stdin_lock = self.child_stdin.lock().expect("Failed to take stdin lock");
          let _drop_stdin = stdin_lock.take();
          println!("Dropping runner stdin.");
        }

        // Now that stdin is closed, we can wait on child
        // TODO: use wait_timeout instead: https://github.com/alexcrichton/wait-timeout
        let mut child = self.child.lock().expect("Failed to get lock on child");
        println!("Waiting for child to exit...");
        let status = child.wait().expect("Failed to wait on child");
        println!("Child exited with {:?}", &status);
        status.code().expect("Failed to get exit code")
    }
}