use {hyper, serde_json};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{Visitor};
use std::{fmt, env, io};

// quick_error generates a lot of the standard error boilerplate
quick_error! {
    #[derive(Debug)]
    pub enum Error {
        /// Errors from the hyper client or server
        HttpError(err: hyper::error::Error) {
            from()
            description(err.description())
            display("HTTP error - {}", err)
        }

        /// Errors reading environment variables
        EnvVarError(err: env::VarError) {
            from()
            description(err.description())
            display("environment var error - {}", err)
        }

        /// Errors serialize type to JSON
        SerdeError(err: serde_json::error::Error) {
            from()
            description(err.description())
            display("JSON error - {}", err)
        }

        IoError(err: io::Error) {
            from()
            description(err.description())
            display("IO error - {}", err)
        }

        /// Errors parsing URLs
        UrlParseError(err: String) {
            description(err)
        }

        /// Error sending notification
        NotificationError(response: hyper::client::Response) {
            description("notification errror")
            display("{}", response.status)
        }

        BadRequest(err: String) {
            description("bad request")
            display("bad request: {}", err)
        }

        UnexpectedExit(code: i32) {
            description("unexpected exit")
            display("exited with code {}", code)
        }

        Unexpected(err: &'static str) {
            description(err)
        }

    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorMessage {
    pub message: String,
    pub error_type: String,

    #[serde(skip_serializing_if="Option::is_none")]
    pub stacktrace: Option<String>,
}

impl ErrorMessage {
    pub fn new(error: Error) -> ErrorMessage {
        ErrorMessage { message: error.to_string(), error_type: ErrorType::SystemExit, stacktrace: None }
    }
}


// Custom serialization/deserialization for ErrorType
// looking into an upstream feature request to remove this boilerplate
// https://github.com/serde-rs/json/issues/290

impl Serialize for ErrorType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            ErrorType::SystemError => serializer.serialize_str("SystemError"),
            ErrorType::SystemExit => serializer.serialize_str("SystemExit"),
            ErrorType::AlgorithmError(ref err_type) => serializer.serialize_str(err_type)
        }

    }
}

impl Deserialize for ErrorType {
    fn deserialize<D>(deserializer: D) -> Result<ErrorType, D::Error>
        where D: Deserializer,
    {
        struct ErrorTypeVisitor;
        impl Visitor for ErrorTypeVisitor {
            type Value = ErrorType;

            fn visit_str<E>(self, value: &str) -> Result<ErrorType, E>
                where E: ::std::error::Error,
            {
                Ok(match value {
                    "SystemError" => ErrorType::SystemError,
                    "SystemExit" => ErrorType::SystemExit,
                    _ => ErrorType::AlgorithmError(value.to_owned()),
                })
            }

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string (usually 'SystemError', 'SystemExit', or 'AlgorithmError')")
            }
        }

        deserializer.deserialize_str(ErrorTypeVisitor)
    }
}
