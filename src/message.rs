use serde_json::{Value};
use std::time::Duration;
use std::env;

use super::error::{Error, ErrorType, ErrorMessage};

pub enum RunnerState {
    Completed(RunnerOutput),
    Exited(Error),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RunnerOutput {
    Success { result: Value, metadata: RunnerMetadata },
    Failure { message: String, stacktrace: Option<String>, error_type: ErrorType },
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
    fn new(duration: Duration, content_type: Option<String>)
        -> Metadata {
        let duration_float = duration.as_secs() as f64 + (duration.subsec_nanos() as f64 / 1_000_000_000f64);
        Metadata {
            duration: duration_float,
            content_type: content_type,
        }
    }
}

impl RunnerState {
    pub fn into_message(self, duration: Duration) -> RunnerMessage {
        match self {
            RunnerState::Completed(RunnerOutput::Success{ result, metadata }) => {
                RunnerMessage::Success{
                    result: result,
                    metadata: Metadata::new(duration, Some(metadata.content_type)),
                }
            }
            RunnerState::Completed(RunnerOutput::Failure{ message, stacktrace, error_type }) => {
                RunnerMessage::Failure {
                    error: ErrorMessage{ message, stacktrace, error_type },
                    metadata: Metadata::new(duration, None),
                }
            }
            RunnerState::Exited(err) => {
                RunnerMessage::Failure {
                    error: ErrorMessage::new(err),
                    metadata: Metadata::new(duration, None),
                }
            }
        }
    }
}

impl RunnerMessage {
    pub fn exited_early(&self) -> bool {
        match *self {
            RunnerMessage::Failure{ ref error, ..} => match error.error_type {
                ErrorType::SystemExit => true,
                _ => false,
            },
            _ => false,
        }
    }
}

impl StatusMessage {
    fn new(health_status: HealthStatus, load_time: Duration) -> StatusMessage {
        let (status, error) = match health_status {
            HealthStatus::Success => ("Successful", None),
            HealthStatus::Failure(err) => ("Failed", Some(err)),
        };
        StatusMessage {
            slot_id: env::var("SLOT_ID").ok(),
            status: status.to_owned(),
            error: error.map(ErrorMessage::new),
            metadata: Metadata {
                duration: load_time.as_secs() as f64 + (load_time.subsec_nanos() as f64 / 1_000_000_000f64),
                content_type: None,
            }
        }
    }

    pub fn success(duration: Duration) -> StatusMessage {
        StatusMessage::new(HealthStatus::Success, duration)
    }

    pub fn failure(err: Error, duration: Duration) -> StatusMessage {
        StatusMessage::new(HealthStatus::Failure(err), duration)
    }
}
