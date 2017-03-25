use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::Url;
use std::time::Duration;
use serde_json::ser;
use serde::Serialize;
use std::thread;
use std::error::Error as StdError;
use super::error::Error;

#[derive(Clone)]
pub struct Notifier {
    url: Url,
}

impl Notifier {
    pub fn parse(url: &str) -> Result<Notifier, Error> {
        match Url::parse(url) {
            Ok(parsed_url) => Ok(Notifier { url: parsed_url }),
            Err(err) => Err(Error::UrlParseError(err.description().to_owned())),
        }
    }

    fn try_notify(&self, body: String, headers: Option<Headers>) -> Result<(), Error> {
        let res = Client::new()
                      .post(self.url.clone())
                      .headers(headers.unwrap_or_else(Headers::new))
                      .header(ContentType::json())
                      .body(&body)
                      .send();

        match res {
            Ok(ref response) if response.status.is_success() => {
                Ok(())
            }
            Ok(response) => {
                println!("Failed to send notification: {}", response.status);
                Err(Error::NotificationError(response))
            }
            Err(err) => {
                println!("Failed to send notification: {}", err);
                Err(err.into())
            }
        }
    }

    pub fn notify<S: Serialize>(&self, message: S, headers: Option<Headers>) -> Result<(), Error> {
        let body = ser::to_string(&message)?;

        let mut i = 1;
        while let Err(err) = self.try_notify(body.clone(), headers.clone()) {
            if i == 3 {
                return Err(err);
            }
            println!("Will retry notification (#{})", i);
            thread::sleep(Duration::from_secs(1));
            i += 1;
        }
        Ok(())
    }
}
