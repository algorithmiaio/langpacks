use serde_json::{Value};
use std::time::Duration;
use std::env;

use super::error::{Error, ErrorMessage, SYSTEM_EXIT};

pub enum RunnerState {
    Completed(RunnerOutput),
    Exited(Error),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RunnerOutput {
    Success { result: Value, metadata: RunnerMetadata },
    Failure { error: ErrorMessage },
}

#[derive(Deserialize)]
pub struct RunnerMetadata { content_type: String }

#[derive(Serialize)]
#[serde(untagged)]
pub enum RunnerMessage {
    Success { result: Value, metadata: Metadata },
    Failure { error: ErrorMessage, metadata: Metadata },
}

#[derive(Serialize)]
pub struct Metadata {
    pub duration: f64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub stdout: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub stderr: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<String>,
}

pub enum HealthStatus {
    Success,
    Failure(Error),
}

#[derive(Serialize)]
pub struct StatusMessage {
    slot_id: Option<String>,
    status: String,
    metadata: Metadata,
    error: Option<ErrorMessage>,
}

impl Metadata {
    fn new(duration: Duration, content_type: Option<String>, stdout: Option<String>, stderr: Option<String>)
        -> Metadata {
        let duration_float = duration.as_secs() as f64 + (duration.subsec_nanos() as f64 / 1_000_000_000f64);
        Metadata {
            duration: duration_float,
            content_type: content_type,
            stdout: stdout,
            stderr: stderr,
        }
    }
}

impl RunnerState {
    pub fn into_message(self, duration: Duration, stdout: Option<String>, stderr: Option<String>) -> RunnerMessage {
        match self {
            RunnerState::Completed(RunnerOutput::Success{ result, metadata }) => {
                RunnerMessage::Success{
                    result: result,
                    metadata: Metadata::new(duration, Some(metadata.content_type), stdout, stderr),
                }
            }
            RunnerState::Completed(RunnerOutput::Failure{ error }) => {
                RunnerMessage::Failure {
                    error: error,
                    metadata: Metadata::new(duration, None, stdout, stderr),
                }
            }
            RunnerState::Exited(err) => {
                // If the err type was an UnexpectedExit then we would already have stdout and
                // stderr from the error message so default to that instead
                // The Error enum doesn't support .clone() so have to do some weird object
                // re-creations here
                let err_stdio = match err {
                    Error::UnexpectedExit(code, stdout, stderr) =>
                        (Error::UnexpectedExit(code, stdout.clone(), stderr.clone()), Some(stdout.clone()), Some(stderr.clone())),
                    _ => (err, stdout, stderr)
                };
                RunnerMessage::Failure {
                    error: ErrorMessage::exit(err_stdio.0),
                    metadata: Metadata::new(duration, None, err_stdio.1, err_stdio.2),
                }
            }
        }
    }
}

impl RunnerMessage {
    pub fn exited_early(&self) -> bool {
        match *self {
            RunnerMessage::Failure{ ref error, ..} => match &*error.error_type {
                SYSTEM_EXIT => true,
                _ => false,
            },
            _ => false,
        }
    }
}

impl StatusMessage {
    fn new(health_status: HealthStatus, load_time: Duration, stdout: Option<String>, stderr: Option<String>) -> StatusMessage {
        let (status, error) = match health_status {
            HealthStatus::Success => ("Successful", None),
            HealthStatus::Failure(err) => ("Failed", Some(err)),
        };
        StatusMessage {
            slot_id: env::var("SLOT_ID").ok(),
            status: status.to_owned(),
            error: error.map(ErrorMessage::exit),
            metadata: Metadata {
                duration: load_time.as_secs() as f64 + (load_time.subsec_nanos() as f64 / 1_000_000_000f64),
                content_type: None,
                stdout: stdout,
                stderr: stderr,
            }
        }
    }

    pub fn success(duration: Duration, stdout: Option<String>, stderr: Option<String>) -> StatusMessage {
        StatusMessage::new(HealthStatus::Success, duration, stdout, stderr)
    }

    pub fn failure(err: Error, duration: Duration, stdout: Option<String>, stderr: Option<String>) -> StatusMessage {
        StatusMessage::new(HealthStatus::Failure(err), duration, stdout, stderr)
    }
}
