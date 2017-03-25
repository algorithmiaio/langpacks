use {std, hyper, serde_json};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

// quick_error generates a lot of the standard error boilerplate
quick_error! {
    #[derive(Debug)]
    pub enum Error {
        /// Errors from the hyper client or server
        HttpError(err: hyper::error::Error) {
            from()
            description(err.description())
            cause(err)
        }

        /// Errors reading environment variables
        EnvVarError(err: std::env::VarError) {
            from()
            description(err.description())
            cause(err)
        }

        /// Errors serialize type to JSON
        SerdeError(err: serde_json::error::Error) {
            from()
            description(err.description())
            cause(err)
        }

        IoError(err: std::io::Error) {
            from()
            description(err.description())
            cause(err)
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

        UnexpectedExit(code: i32, stdout: Option<String>, stderr: Option<String>) {
            description("unexpected exit")
            display("exited with code {}", code)
        }

        Unexpected(err: String) {
            description(err)
        }

    }
}

// Custom serialization of Error
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let error_type = match *self {
            Error::UnexpectedExit(..) => "SystemExit",
            _ => "SystemError",
        };

        let mut struc = serializer.serialize_struct("error", 2)?;
        struc.serialize_field("message", &self.to_string())?;
        struc.serialize_field("error_type", &error_type)?;
        struc.end()
    }
}