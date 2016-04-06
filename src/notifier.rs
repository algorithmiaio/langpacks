use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::Url;
use time::Duration;
use serde_json::ser;
use serde::{self, Serialize, Serializer};
use std::{self, env, thread};
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
                      .headers(headers.unwrap_or(Headers::new()))
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
        let body = try!(ser::to_string(&message));

        let mut i = 1;
        while let Err(err) = self.try_notify(body.clone(), headers.clone()) {
            if i == 3 {
                return Err(err);
            }
            println!("Will retry notification (#{})", i);
            thread::sleep(std::time::Duration::from_secs(1));
            i = i + 1;
        }
        Ok(())
    }
}

pub enum LoadStatus {
    Success,
    Failure(String),
}

pub struct LoadNotification {
    slot_id: Option<String>,
    status: String,
    error: Option<String>,
    load_time: f64,
}

impl LoadNotification {
    pub fn new(load_status: &LoadStatus, load_time: Duration) -> LoadNotification {
        let (status, error) = match load_status {
            &LoadStatus::Success => ("Successful", None),
            &LoadStatus::Failure(ref err) => ("Failed", Some(err.clone())),
        };
        LoadNotification {
            slot_id: env::var("SLOT_ID").ok(),
            status: status.to_owned(),
            error: error,
            load_time: load_time.num_microseconds().unwrap() as f64 / 1_000_000f64,
        }
    }
}

// JSON boilerplate - until compiler plugins are stable to just annotate with #[derive(Serialize)]
impl Serialize for LoadNotification {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        serializer.serialize_map(LoadNotificationMapVisitor { value: self })
    }
}
struct LoadNotificationMapVisitor<'a> {
    value: &'a LoadNotification,
}
impl<'a> serde::ser::MapVisitor for LoadNotificationMapVisitor<'a> {
    fn visit<S: Serializer>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> {
        try!(serializer.serialize_map_elt("slot_id", &self.value.slot_id));
        try!(serializer.serialize_map_elt("status", &self.value.status));
        try!(serializer.serialize_map_elt("load_time", &self.value.load_time));
        if let Some(ref error) = self.value.error {
            try!(serializer.serialize_map_elt("error", error));
        }
        Ok(None)
    }
}
