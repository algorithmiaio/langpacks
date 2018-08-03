use {hyper, serde_json};
use std::{env, io};

pub const SYSTEM_ERROR: &'static str = "SystemError";
pub const SYSTEM_EXIT: &'static str = "SystemExit";

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

    #[serde(default = "default_error_type")]
    pub error_type: String,

    #[serde(skip_serializing_if="Option::is_none")]
    pub stacktrace: Option<String>,
}

fn default_error_type() -> String {
    "AlgorithmError".to_string()
}

impl ErrorMessage {
    pub fn exit(error: Error) -> ErrorMessage {
        ErrorMessage { message: error.to_string(), error_type: SYSTEM_EXIT.to_owned(), stacktrace: None }
    }
}

