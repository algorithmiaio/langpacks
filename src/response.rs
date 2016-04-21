use serde_json::Value;
use serde::{self, Serialize, Serializer};
use std::collections::BTreeMap;
use std::time::Duration;

use super::error::Error;

pub enum RunnerOutput {
    Completed(Value),
    Exited(Value),
}

pub struct ErrorResponse {
    pub metadata: Option<ResponseMetadata>,
    pub error: Error
}

impl ErrorResponse {
    pub fn from_error(err: Error) -> ErrorResponse {
        ErrorResponse {
            metadata: None,
            error: err,
        }
    }
}

pub struct ResponseMetadata {
    pub duration: f64,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    // content_type is never added by LangServer/LangRunner, only by the pipe process
}


// JSON boilerplate for ErrorResponse - until #[derive(Serialize)] is stabilized
impl Serialize for ErrorResponse {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        serializer.serialize_map(ErrorResponseMapVisitor { value: self })
    }
}
struct ErrorResponseMapVisitor<'a> {
    value: &'a ErrorResponse,
}

impl<'a> serde::ser::MapVisitor for ErrorResponseMapVisitor<'a> {
    fn visit<S: Serializer>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> {
        try!(serializer.serialize_map_elt("error", &self.value.error));
        if let Some(ref metadata) = self.value.metadata {
            try!(serializer.serialize_map_elt("metadata", &metadata));
        }
        Ok(None)
    }
}


// JSON boilerplate for ResponseMetadata - until #[derive(Serialize)] is stabilized
impl Serialize for ResponseMetadata {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        serializer.serialize_map(ResponseMetadataMapVisitor { value: self })
    }
}
struct ResponseMetadataMapVisitor<'a> {
    value: &'a ResponseMetadata,
}
impl<'a> serde::ser::MapVisitor for ResponseMetadataMapVisitor<'a> {
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


impl RunnerOutput {
    fn value_mut(&mut self) -> &mut Value {
        match self {
            &mut RunnerOutput::Completed(ref mut value) => value,
            &mut RunnerOutput::Exited(ref mut value) => value,
        }
    }

    pub fn set_metadata(&mut self, duration: Duration, stdout: String, stderr: String) {
        let mut metadata = self.metadata_mut();
        let duration_float = duration.as_secs() as f64 + (duration.subsec_nanos() as f64 / 1_000_000_000f64);
        metadata.insert(s!("duration"), Value::F64(duration_float));
        if !stdout.is_empty() {
            metadata.insert(s!("stdout"), Value::String(stdout));
        }
        if !stderr.is_empty() {
            metadata.insert(s!("stderr"), Value::String(stderr));
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
