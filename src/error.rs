use std::error::Error as StdError;
use {std, hyper, serde_json};

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

        Unexpected(err: String) {
            description(err)
        }

    }
}
