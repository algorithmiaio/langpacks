use std::error::Error as StdError;
use {std, hyper, serde_json};
use serde::{self, Serialize, Serializer};

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

        UnexpectedExit(code: i32) {
            description("unexpected exit")
            display("exited with code {}", code)
        }

        Unexpected(err: String) {
            description(err)
        }

    }
}

// JSON boilerplate for Error - until #[derive(Serialize)] is stabilized
impl Serialize for Error {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        serializer.serialize_map(ErrorMapVisitor { value: self })
    }
}
struct ErrorMapVisitor<'a> {
    value: &'a Error,
}
impl<'a> serde::ser::MapVisitor for ErrorMapVisitor<'a> {
    fn visit<S: Serializer>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> {
        let error_type = match self.value {
            &Error::UnexpectedExit(_) => "SystemExit",
            _ => "SystemError",
        };
        try!(serializer.serialize_map_elt("message", &self.value.to_string()));
        try!(serializer.serialize_map_elt("error_type", &error_type));
        Ok(None)
    }
}