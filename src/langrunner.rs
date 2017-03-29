use serde::ser::Serialize;
use serde_json::{ser, to_value, Deserializer};
use serde_json::Value;
use std::env;
use std::io::{self, Write, BufRead, BufReader};
use std::ffi::CString;
use std::fs::File;
use std::path::Path;
use std::{process, thread};
use std::process::*;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::{Duration, Instant};
use libc;
use wait_timeout::ChildExt;

use super::error::Error;
use super::message::{ErrorMessage, RunnerOutput};

const ALGOOUT: &'static str = "/tmp/algoout";
const UNKNOWN_EXIT: i32 = -99;
const LOG_IDENTIFIER: &'static str = "LANGRUNNER";

type RunnerResult = Result<Value, Error>;

// Wrapper around the LangRunnerProcess that uses channels to wait on things
// because several aspects of the LangRunnerProcess are blocking
pub struct LangRunner {
    runner: Arc<RwLock<LangRunnerProcess>>,
    tx: Sender<RunnerResult>,
    rx: Receiver<RunnerResult>,
}

// Struct to manage the `bin/pipe` process
struct LangRunnerProcess {
    stdout_lines: Arc<Mutex<Vec<String>>>,
    stderr_lines: Arc<Mutex<Vec<String>>>,
    stdin: Option<ChildStdin>,
    child: Mutex<Child>,
    exit_status: Mutex<Option<i32>>,
    request_id: Arc<RwLock<Option<String>>>,
}

// This blocks until output is available on ALGOOUT
fn get_next_algoout_value(request_id: Arc<RwLock<Option<String>>>) -> Result<Value, Error> {
    let req_id = request_id.read().expect("failed to get read handle on request_id for stdout reading").clone().unwrap_or("-".to_owned());
    // Note: Opening a FIFO read-only pipe blocks until a writer opens it.
    info!("{} {} Opening /tmp/algoout FIFO...", LOG_IDENTIFIER, req_id);
    let algoout = File::open(ALGOOUT)?;


    // Read and deserialize the single next JSON Value on ALGOOUT
    info!("{} {} Deserializing algoout stream...", LOG_IDENTIFIER, req_id);
    let mut algoout_stream = Deserializer::from_reader(algoout).into_iter::<Value>();
    match algoout_stream.next() {
        Some(next) => match next {
            Ok(out) => Ok(out),
            Err(err) => {
                error!("{} {} Failed to deserialize next JSON value from stream: {}", LOG_IDENTIFIER, "-", err);
                Err(err.into())
            }
        },
        None => Err(Error::Unexpected("No more JSON to read from the stream".to_owned())),
    }
}

fn get_and_clear_lines(vec: Arc<Mutex<Vec<String>>>) -> Option<String> {
    let mut lines = vec.lock().expect("Failed to get lock on lines");
    if lines.len() > 0 {
        let mut algo_out = lines.join("\n");
        if algo_out.chars().last() == Some('\n') {
            let _ = algo_out.pop();
        }
        lines.clear();
        Some(algo_out)
    } else {
        None
    }
}

impl LangRunner {
    // Start the runner process, initialize channels, and begin monitoring the runner process for exit
    pub fn start() -> Result<LangRunner, Error> {
        if !Path::new(ALGOOUT).exists() {
            let mode = 0o644;
            let location = CString::new(ALGOOUT).unwrap();
            unsafe {
                match libc::mkfifo(location.as_ptr(), mode) {
                    0 => (),
                    _ => panic!("Unable to create algoout fifo: {}", io::Error::last_os_error()),
                }
            }
        }

        let runner = LangRunnerProcess::start()?;
        let (tx, rx) = channel();
        let lr = LangRunner {
            runner: Arc::new(RwLock::new(runner)),
            rx: rx,
            tx: tx,
        };
        lr.monitor();
        Ok(lr)
    }

    // Monitor runner - notify receiver channel if exit is encountered
    fn monitor(&self) {
        let tx = self.tx.clone();
        let arc_runner = self.runner.clone();
        thread::spawn(move || {
            loop {
                {
                    let runner = arc_runner.read().expect("Failed to acquire read lock on runner");
                    if let Some(code) = runner.check_exited() {
                        warn!("{} {} LangRunner monitor thread detected exit: {}", LOG_IDENTIFIER, "-", code);
                        if let Err(err) = tx.send(Err(Error::UnexpectedExit(code, None, None))) {
                            error!("{} {} FATAL: Channel receiver disconnected unexpectedly: {}", LOG_IDENTIFIER, "-", err);
                            process::exit(code); // Don't want to just panic a single thread and hang
                        }
                        break;
                    };
                }
                thread::sleep(Duration::from_millis(500));
            }
        });
    }

    pub fn write<T: Serialize>(&mut self, input: &T) -> Result<(), Error> {
        let mut runner = self.runner.write().expect("Failed to acquire write lock on runner");
        runner.write(input)
    }

    pub fn set_request_id(&mut self, request_id: Option<String>) {
        let runner = self.runner.write().expect("Failed to acquire write lock for runner request_id");
        let mut write_handle = runner.request_id.write().expect("Failed to get write lock for request_id");
        *write_handle = request_id;
    }

    pub fn get_request_id(&self) -> String {
        let runner = self.runner.read().expect("Failed to acquire read lock for runner request_id");
        let read_handle = runner.request_id.read().expect("Failed to get read lock for request_id");
        read_handle.clone().unwrap_or("-".to_owned())
    }

    pub fn wait_for_response_or_exit(&mut self) -> RunnerOutput {
        let tx = self.tx.clone();

        let start = Instant::now();
        let runner = self.runner.read().expect("Failed to acquire read lock for runner");
        let arc_request_id = runner.request_id.clone();
        thread::spawn(move || {
            if let Err(err) = tx.send(get_next_algoout_value(arc_request_id)) {
                error!("{} {} FATAL: Channel receiver disconnected unexpectedly: {}", LOG_IDENTIFIER, "-", err);
                process::exit(UNKNOWN_EXIT); // Don't want to just panic a single thread and hang
            }
        });

        // Block until receiving message from `get_next_algoout_value` or the monitor thread
        let received = self.rx.recv().expect("Channel sender disconnected unexpectedly");
        let duration = start.elapsed();

        let mut runner_output = match received {
            Ok(response) => RunnerOutput::Completed(response),
            Err(err) => {
                error!("{} {} Wait encountered an error: {}", LOG_IDENTIFIER, "-", err);
                let response = ErrorMessage::from_error(err);
                RunnerOutput::Exited(to_value(&response).expect("RunnerOutput serialization failed"))
            }
        };

        let (stdout, stderr) = self.consume_stdio();

        // Augment output with duration and stdout
        runner_output.set_metadata(duration, stdout, stderr);
        runner_output
    }

    pub fn consume_stdio(&self) -> (Option<String>, Option<String>) {
        let runner = self.runner.read().expect("Failed to acquire read lock on runner");
        (runner.consume_stdout(), runner.consume_stderr())
    }

    pub fn check_exited(&self) -> Option<i32> {
        let runner = self.runner.read().expect("Failed to acquire read lock on runner");
        runner.check_exited()
    }

    pub fn stop(&mut self) -> i32 {
        match self.check_exited() {
            Some(code) => code,
            None => {
                let mut runner = self.runner
                                     .write()
                                     .expect("Failed to acquire write lock on runner");
                runner.stop()
            }
        }
    }
}

impl LangRunnerProcess {
    fn start() -> Result<LangRunnerProcess, Error> {
        let mut path = env::current_dir()?;
        path.push("bin/pipe");

        let request_id = Arc::new(RwLock::new(None));

        let mut child = Command::new(&path)
                                 .stdin(Stdio::piped())
                                 .stdout(Stdio::piped())
                                 .stderr(Stdio::piped())
                                 .spawn()?;

        info!("{} {} Running PID {}: {}", LOG_IDENTIFIER, "-", child.id(), path.to_string_lossy());

        let stdin = child.stdin
                              .take()
                              .ok_or_else(|| Error::Unexpected(s!("Failed to open runner's STDIN")))?;
        let stdout = child.stdout
                               .take()
                               .ok_or_else(|| Error::Unexpected(s!("Failed to open runner's STDOUT")))?;
        let stderr = child.stderr
                               .take()
                               .ok_or_else(|| Error::Unexpected(s!("Failed to open runner's STDERR")))?;

        let child_stderr = Arc::new(Mutex::new(Vec::new()));
        // Spawn a thread to collect algorithm stderr - we do this here so we shouldn't get get stuck
        // waiting for stuff to be read from stderr when we are loading the algorithm
        let arc_stderr = child_stderr.clone();
        let arc_request_id_err = request_id.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line_result in reader.lines() {
                let req_id = arc_request_id_err.read().expect("failed to get read handle on request_id for stderr reading").clone().unwrap_or("-".to_owned());
                match line_result {
                    Ok(line) => match arc_stderr.lock() {
                        Ok(mut lines) => {
                            info!("{} {} {}", "ALGOERR", req_id, line);
                            lines.push(line);
                        }
                        Err(err) => error!("{} {} Failed to get lock on stderr lines: {}", LOG_IDENTIFIER, req_id, err),
                    },
                    Err(err) => error!("{} {} Failed to read line: {}", LOG_IDENTIFIER, req_id, err),
                }
            }
        });

        let mut reader = BufReader::new(stdout);
        let mut collected_stdout = String::new();
        loop {
            let mut line = Vec::new();
            match reader.read_until(b'\n', &mut line) {
                Ok(0) => {
                    info!("{} {} Reached stdout EOF", LOG_IDENTIFIER, "-");
                    // Wait for exit return UnexpectedExit with stdout & stderr
                    let code = child.wait().ok().and_then(|exit| exit.code()).unwrap_or(UNKNOWN_EXIT);
                    //let mut collected_stderr = self.consume_stderr()
                    //let bytes = stderr.read_to_string(&mut collected_stderr).unwrap_or(0);
                    let collected_stderr = match get_and_clear_lines(child_stderr.clone()) {
                        Some(cs) => cs,
                        _ => String::new()
                    };

                    if !collected_stderr.is_empty() {
                        let _ = io::stderr().write(collected_stderr.as_bytes());
                    }

                    let stdout_opt = if collected_stdout.is_empty() {
                        None
                    } else {
                        Some(collected_stdout)
                    };
                    let stderr_opt = if collected_stderr.is_empty() {
                        None
                    } else {
                        Some(collected_stderr)
                    };
                    return Err(Error::UnexpectedExit(code, stdout_opt, stderr_opt));
                }
                Ok(_) => {
                    let line_str = String::from_utf8_lossy(&line);
                    let req_id = request_id.read().expect("failed to get read handle on request_id for stderr reading").clone().unwrap_or("-".to_owned());
                    collected_stdout.push_str(&line_str.replace("PIPE_INIT_COMPLETE\n",""));
                    if line_str.contains("PIPE_INIT_COMPLETE") {
                        info!("{} {} {}", LOG_IDENTIFIER, req_id, &line_str.replace("\n",""));
                        break;
                    }
                    else {
                        info!("{} {} {}", "ALGOOUT", req_id, &line_str.replace("\n",""));
                    }
                }
                Err(err) => {
                    error!("{} {} Failed to read child stdout: {}", LOG_IDENTIFIER, "-", err);
                    return Err(err.into());
                }
            }
        }

        let child_stdout = Arc::new(Mutex::new(vec![collected_stdout]));
        // Spawn a thread to collect algorithm stdout
        let arc_stdout = child_stdout.clone();
        let arc_request_id_out = request_id.clone();
        thread::spawn(move || {
            for line_result in reader.lines() {
                let req_id = arc_request_id_out.read().expect("failed to get read handle on request_id for stdout reading").clone().unwrap_or("-".to_owned());
                match line_result {
                    Ok(line) => match arc_stdout.lock() {
                        Ok(mut lines) => {
                            info!("{} {} {}", "ALGOOUT", req_id, line);
                            lines.push(line);
                        }
                        Err(err) => error!("{} {} Failed to get lock on stdout lines: {}", LOG_IDENTIFIER, req_id, err),
                    },
                    Err(err) => error!("{} {} Failed to read line: {}", LOG_IDENTIFIER, req_id, err),
                }
            }
        });

        Ok(LangRunnerProcess {
            child: Mutex::new(child),
            stdin: Some(stdin),
            stdout_lines: child_stdout,
            stderr_lines: child_stderr,
            exit_status: Mutex::new(None),
            request_id: request_id.clone()
        })
    }

    pub fn write<T: Serialize>(&mut self, input: &T) -> Result<(), Error> {
        match self.stdin.as_mut() {
            Some(mut stdin) => {
                let req_id = self.request_id.read().expect("failed to get read handle on request_id for stdout reading").clone().unwrap_or("-".to_owned());
                info!("{} {} Sending data to runner stdin", LOG_IDENTIFIER, req_id);
                ser::to_writer(&mut stdin, &input)?;
                stdin.write_all(b"\n")?;
                Ok(())
            }
            None => {
                Err(Error::Unexpected("cannot write to closed runner stdin".to_owned()))
            }
        }
    }

    // This returns available stdout without blocking
    pub fn consume_stdout(&self) -> Option<String> {
        get_and_clear_lines(self.stdout_lines.clone())
    }

    // This returns available stderr without blocking
    pub fn consume_stderr(&self) -> Option<String> {
        get_and_clear_lines(self.stderr_lines.clone())
    }

    pub fn check_exited(&self) -> Option<i32> {
        // Check if we've already stored the exit code
        // Also holding lock on self.exit_status to ensure wait_timeout is called safely between threads
        let mut exit_status = self.exit_status.lock().expect("Failed to take exit status lock");
        if exit_status.is_some() {
            return Some(exit_status.unwrap());
        }

        // Now let's do a short wait just to see if the process has exited
        let mut child = self.child.lock().expect("Failed to get lock on runner");
        match child.wait_timeout(Duration::from_millis(10)) {
            Err(err) => {
                error!("{} {} Error waiting for runner: {}", LOG_IDENTIFIER, "-", err);
                *exit_status = Some(UNKNOWN_EXIT);
                Some(UNKNOWN_EXIT)
            }
            Ok(Some(exit)) => {
                info!("{} {} Runner exited - {}", LOG_IDENTIFIER, "-", exit);
                let code = exit.code().unwrap_or(UNKNOWN_EXIT);
                *exit_status = Some(code);
                Some(code)
            }
            Ok(None) => None, // Still alive
        }
    }

    pub fn stop(&mut self) -> i32 {
        // Check if we've already stored the exit code
        // Also holding lock on self.exit_status to ensure wait_timeout is called safely between threads
        let mut exit_status = self.exit_status.lock().expect("Failed to take exit status lock");
        if exit_status.is_some() {
            return exit_status.unwrap();
        }

        // Mutably `take` child_stdin out of `self` and drop it
        if let Some(_drop_stdin) = self.stdin.take() {
            info!("{} {} Sending EOF to runner stdin.", LOG_IDENTIFIER, "-");
        } // _drop_stdin goes out of scope here which results in EOF

        // Now that stdin is closed, we can wait on child
        info!("{} {} Waiting for runner to exit...", LOG_IDENTIFIER, "-");
        let mut child = self.child.lock().expect("Failed to get lock on runner");
        let code = match child.wait_timeout(Duration::from_secs(3)) {
            Err(err) => {
                error!("{} {} Error waiting for runner: {}", LOG_IDENTIFIER, "-", err);
                UNKNOWN_EXIT
            }
            Ok(Some(exit)) => {
                info!("{} {} Runner exited - {}", LOG_IDENTIFIER, "-", exit);
                exit.code().unwrap_or(UNKNOWN_EXIT)
            }
            Ok(None) => {
                warn!("{} {} Runner did not exit. Killing.", LOG_IDENTIFIER, "-");
                if let Err(err) = child.kill() {
                    error!("{} {} Failed to kill runner: {}", LOG_IDENTIFIER, "-", err);
                }
                UNKNOWN_EXIT
            }
        };

        // Store the exit status
        *exit_status = Some(code);
        code
    }
}
