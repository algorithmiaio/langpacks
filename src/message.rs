use serde_json::Value;
use serde::{self, Serialize, Serializer};
use std::collections::BTreeMap;
use std::time::Duration;
use std::env;

use super::error::Error;
use super::notifier::HealthStatus;

pub enum RunnerOutput {
    Completed(Value),
    Exited(Value),
}

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

pub struct Metadata {
    pub duration: f64,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    // content_type is never added by LangServer/LangRunner, only by the pipe process
}

pub struct StatusMessage {
    slot_id: Option<String>,
    status: String,
    metadata: Metadata,
    error: Option<Error>,
}

impl StatusMessage {
    pub fn new(load_status: HealthStatus, load_time: Duration, stdout: Option<String>, stderr: Option<String>) -> StatusMessage {
        let (status, error) = match load_status {
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
        metadata.insert(s!("duration"), Value::F64(duration_float));
        if let Some(value) = stdout {
            metadata.insert(s!("stdout"), Value::String(value));
        }
        if let Some(value) = stderr {
            metadata.insert(s!("stderr"), Value::String(value));
        }
    }

    fn metadata_mut(&mut self) -> &mut BTreeMap<String, Value> {
        let mut metadata = match self.value_mut().as_object_mut() {
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

/*
* JSON serialization boilerplate below
* Most of which could simply use #[derive(Serialize)] when stabilized
*/

// JSON boilerplate for ErrorMessage
impl Serialize for ErrorMessage {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        serializer.serialize_map(ErrorMessageMapVisitor { value: self })
    }
}
struct ErrorMessageMapVisitor<'a> {
    value: &'a ErrorMessage,
}

impl<'a> serde::ser::MapVisitor for ErrorMessageMapVisitor<'a> {
    fn visit<S: Serializer>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> {
        try!(serializer.serialize_map_elt("error", &self.value.error));
        if let Some(ref metadata) = self.value.metadata {
            try!(serializer.serialize_map_elt("metadata", &metadata));
        }
        Ok(None)
    }
}


// JSON boilerplate for Metadata
impl Serialize for Metadata {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        serializer.serialize_map(MetadataMapVisitor { value: self })
    }
}
struct MetadataMapVisitor<'a> {
    value: &'a Metadata,
}
impl<'a> serde::ser::MapVisitor for MetadataMapVisitor<'a> {
    fn visit<S: Serializer>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> {
        try!(serializer.serialize_map_elt("duration", &self.value.duration));
        if let Some(ref stdout) = self.value.stdout {
            try!(serializer.serialize_map_elt("stdout", stdout));
        }
        if let Some(ref stderr) = self.value.stderr {
            try!(serializer.serialize_map_elt("stderr", stderr));
        }
        Ok(None)
    }
}


// JSON boilerplate for StatusMessage
impl Serialize for StatusMessage {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        serializer.serialize_map(StatusMessageMapVisitor { value: self })
    }
}
struct StatusMessageMapVisitor<'a> {
    value: &'a StatusMessage,
}
impl<'a> serde::ser::MapVisitor for StatusMessageMapVisitor<'a> {
    fn visit<S: Serializer>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> {
        try!(serializer.serialize_map_elt("slot_id", &self.value.slot_id));
        try!(serializer.serialize_map_elt("status", &self.value.status));
        try!(serializer.serialize_map_elt("metadata", &self.value.metadata));
        if let Some(ref error) = self.value.error {
            try!(serializer.serialize_map_elt("error", error));
        }
        Ok(None)
    }
}
