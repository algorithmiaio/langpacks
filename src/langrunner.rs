use serde::ser::Serialize;
use serde_json::{ser, Deserializer};
use std::{env, mem};
use std::io::{self, Write, BufRead, BufReader};
use std::ffi::CString;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::{process, thread};
use std::process::*;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::{Duration, Instant};
use libc;
use wait_timeout::ChildExt;

use super::error::Error;
use super::message::{RunnerMessage, RunnerState, RunnerOutput};

const ALGOOUT: &'static str = "/tmp/algoout";
const UNKNOWN_EXIT: i32 = -99;
const LOG_IDENTIFIER: &'static str = "LANGRUNNER";

type RunnerResult = Result<RunnerOutput, Error>;

// Wrapper around the LangRunnerProcess that uses channels to wait on things
// because several aspects of the LangRunnerProcess are blocking
pub struct LangRunner {
    runner: Arc<RwLock<LangRunnerProcess>>,
    tx: Sender<RunnerResult>,
    rx: Receiver<RunnerResult>,
}

// Struct to manage the `bin/pipe` process
struct LangRunnerProcess {
    stdin: Option<ChildStdin>,
    child: Mutex<Child>,
    stdout_buf: Arc<Mutex<String>>,
    stderr_buf: Arc<Mutex<String>>,
    exit_status: Mutex<Option<i32>>,
    request_id: Arc<RwLock<Option<String>>>,
}

// This blocks until output is available on ALGOOUT
fn get_next_algoout_value(request_id: Arc<RwLock<Option<String>>>) -> Result<RunnerOutput, Error> {
    let req_id = request_id.read()
        .expect("failed to get read handle on request_id for stdout reading")
        .clone().unwrap_or_else(|| "-".to_owned());
    // Note: Opening a FIFO read-only pipe blocks until a writer opens it.
    info!("{} {} Opening /tmp/algoout FIFO...", LOG_IDENTIFIER, req_id);
    let algoout = File::open(ALGOOUT)?;

    // Read and deserialize the single next JSON Value on ALGOOUT
    info!("{} {} Deserializing algoout stream...", LOG_IDENTIFIER, req_id);
    let mut algoout_stream = Deserializer::from_reader(algoout).into_iter();
    match algoout_stream.next() {
        Some(next) => Ok(next?),
        None => Err(Error::Unexpected("No more JSON to read from the stream")),
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
                        let (stdout, stderr) = runner.take_stdio();
                        if let Err(err) = tx.send(Err(Error::UnexpectedExit(code, stdout, stderr))) {
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

    pub fn take_stdio(&self) -> (String, String) {
        let runner = self.runner.read().expect("Failed to acquire read lock on runner");
        runner.take_stdio()
    }

    pub fn set_request_id(&mut self, request_id: Option<String>) {
        let runner = self.runner.write().expect("Failed to acquire write lock for runner request_id");
        let mut write_handle = runner.request_id.write().expect("Failed to get write lock for request_id");
        *write_handle = request_id;
    }

    pub fn wait_for_response_or_exit(&mut self) -> RunnerMessage {
        let tx = self.tx.clone();

        let start = Instant::now();
        let result = {
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

            let runner_state = match received {
                Ok(response) => RunnerState::Completed(response),
                Err(err) => {
                    error!("{} {} Wait encountered an error: {}", LOG_IDENTIFIER, "-", err);
                    RunnerState::Exited(err)
                }
            };

            // Augment output with duration and stdout
            let stdio = self.take_stdio();
            runner_state.into_message(duration, Some(stdio.0), Some(stdio.1))
        };

        // We are now done with a request, we can set the request_id to none
        self.set_request_id(None);

        result
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
        let mut old_path = env::current_dir()?;
        old_path.push("bin/pipe");

        // New IPA algorithms have an algorithmia-pipe that is installed at /usr/local/bin/
        // We use that first. Then we fall back to the /opt/algoritm/bin/pipe if it exists
        let new_path = PathBuf::from("/usr/local/bin/algorithmia-pipe");
        let path = if new_path.exists() {
            new_path
        } else {
            old_path
        };

        let request_id = Arc::new(RwLock::new(None));
        let stdout_buf = Arc::new(Mutex::new(String::new()));
        let stderr_buf = Arc::new(Mutex::new(String::new()));

        let mut child = Command::new(&path)
                                 .stdin(Stdio::piped())
                                 .stdout(Stdio::piped())
                                 .stderr(Stdio::piped())
                                 .spawn()?;

        info!("{} {} Running PID {}: {}", LOG_IDENTIFIER, "-", child.id(), path.to_string_lossy());

        let stdin = child.stdin
                              .take()
                              .ok_or_else(|| Error::Unexpected("Failed to open runner's STDIN"))?;
        let stdout = child.stdout
                               .take()
                               .ok_or_else(|| Error::Unexpected("Failed to open runner's STDOUT"))?;
        let stderr = child.stderr
                               .take()
                               .ok_or_else(|| Error::Unexpected("Failed to open runner's STDERR"))?;

        // Spawn a thread to handle algorithm stderr - we do this here so we shouldn't get get stuck
        // waiting for stuff to be read from stderr when we are loading the algorithm
        let arc_request_id_err = request_id.clone();
        let arc_stderr_buf = stderr_buf.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line_result in reader.lines() {
                let req_id = arc_request_id_err
                    .read().expect("failed to get read handle on request_id for stderr reading")
                    .clone().unwrap_or_else(|| "-".to_owned());
                match line_result {
                    Ok(line) => {
                        match arc_stderr_buf.lock() {
                            Ok(mut lines) => {
                                lines.push_str(&line);
                                lines.push('\n');
                            }
						    Err(err) => error!("{} {} Failed to get lock on stderr buffer: {}", LOG_IDENTIFIER, req_id, err),
                        }
                        info!("{} {} {}", "ALGOERR", req_id, line);
                    },
                    Err(err) => error!("{} {} Failed to read line: {}", LOG_IDENTIFIER, req_id, err),
                }
            }
        });

        let mut reader = BufReader::new(stdout);
        let arc_stderr_buf = stderr_buf.clone();
        let arc_stdout_buf = stdout_buf.clone();
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => {
                    info!("{} {} Reached stdout EOF", LOG_IDENTIFIER, "-");
                    // Wait for exit return UnexpectedExit
                    let code = child.wait().ok().and_then(|exit| exit.code()).unwrap_or(UNKNOWN_EXIT);
                    return Err(Error::UnexpectedExit(code, arc_stdout_buf.lock().unwrap().clone(), arc_stderr_buf.lock().unwrap().clone()));
                }
                Ok(_) => {
                    let _newline = line.pop();
                    let req_id = request_id.read()
                        .expect("failed to get read handle on request_id for stderr reading")
                        .clone().unwrap_or_else(|| "-".to_owned());
                    if line.contains("PIPE_INIT_COMPLETE") {
                        info!("{} {} {}", LOG_IDENTIFIER, req_id, &line);
                        break;
                    } else {
                        match arc_stdout_buf.lock() {
                            Ok(mut lines) => {
                                lines.push_str(&line);
                                lines.push('\n');
                            }
						    Err(err) => error!("{} {} Failed to get lock on stdout buffer: {}", LOG_IDENTIFIER, req_id, err),
                        }
                        info!("{} {} {}", "ALGOOUT", req_id, &line);
                    }
                }
                Err(err) => {
                    error!("{} {} Failed to read child stdout: {}", LOG_IDENTIFIER, "-", err);
                    return Err(err.into());
                }
            }
        }

        // Spawn a thread to handle algorithm stdout
        let arc_request_id_out = request_id.clone();
        thread::spawn(move || {
            for line_result in reader.lines() {
                let req_id = arc_request_id_out
                    .read().expect("failed to get read handle on request_id for stdout reading")
                    .clone().unwrap_or_else(|| "-".to_owned());
                match line_result {
                    Ok(line) => {
                        match arc_stdout_buf.lock() {
                            Ok(mut lines) => lines.push_str(&line),
						    Err(err) => error!("{} {} Failed to get lock on stdout buffer: {}", LOG_IDENTIFIER, req_id, err),
                        }
                        info!("{} {} {}", "ALGOOUT", req_id, line);
                    }
                    Err(err) => error!("{} {} Failed to read line: {}", LOG_IDENTIFIER, req_id, err),
                }
            }
        });

        Ok(LangRunnerProcess {
            child: Mutex::new(child),
            stdin: Some(stdin),
            stdout_buf,
            stderr_buf,
            exit_status: Mutex::new(None),
            request_id: request_id.clone()
        })
    }

    pub fn write<T: Serialize>(&mut self, input: &T) -> Result<(), Error> {
        match self.stdin.as_mut() {
            Some(mut stdin) => {
                let req_id = self.request_id.read()
                    .expect("failed to get read handle on request_id for stdout reading")
                    .clone().unwrap_or_else(|| "-".to_owned());
                info!("{} {} Sending data to runner stdin", LOG_IDENTIFIER, req_id);
                ser::to_writer(&mut stdin, &input)?;
                stdin.write_all(b"\n")?;
                Ok(())
            }
            None => {
                Err(Error::Unexpected("cannot write to closed runner stdin"))
            }
        }
    }

    // Reads the buffered stdout/stderr, emptying the buffer in the process
    pub fn take_stdio(&self) -> (String, String) {
        // Instead of copying data, we allocate new buffers, swap pointers, and return old buffers
        let mut swap_stdout = String::new();
        let mut swap_stderr = String::new();

        mem::swap(&mut *self.stdout_buf.lock().unwrap(), &mut swap_stdout);
        mem::swap(&mut *self.stderr_buf.lock().unwrap(), &mut swap_stderr);

        (swap_stdout, swap_stderr)
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
