use serde_json::{Value, Map};
use std::time::Duration;
use std::env;

use super::error::Error;

pub enum RunnerOutput {
    Completed(Value),
    Exited(Value),
}

#[derive(Serialize)]
pub struct ErrorMessage {
    pub metadata: Option<Metadata>,
    pub error: Error
}

impl ErrorMessage {
    pub fn from_error(err: Error) -> ErrorMessage {
        ErrorMessage {
            metadata: None,
            error: err,
        }
    }
}

#[derive(Serialize)]
pub struct Metadata {
    pub duration: f64,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    // content_type is never added by LangServer/LangRunner, only by the pipe process
}

pub enum HealthStatus {
    Success,
    Failure(Error)
}

#[derive(Serialize)]
pub struct StatusMessage {
    slot_id: Option<String>,
    status: String,
    metadata: Metadata,
    error: Option<Error>,
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
            error: error,
            metadata: Metadata {
                duration: load_time.as_secs() as f64 + (load_time.subsec_nanos() as f64 / 1_000_000_000f64),
                stdout: stdout,
                stderr: stderr,
            }
        }
    }

    pub fn success(duration: Duration) -> StatusMessage {
        StatusMessage::new(HealthStatus::Success, duration, None, None)
    }

    pub fn failure(err: Error, duration: Duration) -> StatusMessage {
        let (stdout, stderr) = match err {
            Error::UnexpectedExit(_, ref stdout, ref stderr) => (stdout.clone(), stderr.clone()),
            _ => (None, None),
        };
        StatusMessage::new(HealthStatus::Failure(err), duration, stdout, stderr)
    }
}

impl RunnerOutput {
    fn value_mut(&mut self) -> &mut Value {
        match self {
            &mut RunnerOutput::Completed(ref mut value) => value,
            &mut RunnerOutput::Exited(ref mut value) => value,
        }
    }

    pub fn set_metadata(&mut self, duration: Duration, stdout: Option<String>, stderr: Option<String>) {
        let mut metadata = self.metadata_mut();
        let duration_float = duration.as_secs() as f64 + (duration.subsec_nanos() as f64 / 1_000_000_000f64);
        metadata.insert(s!("duration"), json!(duration_float));
        if let Some(value) = stdout {
            metadata.insert(s!("stdout"), json!(value));
        }
        if let Some(value) = stderr {
            metadata.insert(s!("stderr"), json!(value));
        }
    }

    fn metadata_mut(&mut self) -> &mut Map<String, Value> {
        let mut metadata = match self.value_mut().as_object_mut() {
            Some(map) => map.entry("metadata").or_insert_with(|| json!(Map::new())),
            None => panic!("Output not a valid structure"),
        };
        metadata.as_object_mut().expect("metadata is not an object")
    }
}
