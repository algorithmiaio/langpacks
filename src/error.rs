use std::error::Error as StdError;
use std;
use hyper;

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
        VarError(err: std::env::VarError) {
            from()
            description(err.description())
            cause(err)
        }

    }
}

