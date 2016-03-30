use hyper;
use hyper::client::Client;
use hyper::client::response::Response;
use hyper::header::{Headers, ContentType};
use hyper::Url;
use time::Duration;
use serde_json::ser;
use serde::{self, Serialize, Serializer};
use std::env;
use std::error::Error;

#[derive(Clone)]
pub struct Notifier {
    url: Url
}

impl Notifier {
    pub fn parse(url: &str) -> Result<Notifier, String> {
        match Url::parse(url) {
            Ok(parsed_url) => Ok(Notifier {  url: parsed_url }),
            Err(err) => Err(err.description().to_owned()),
        }
    }

    pub fn notify<S: Serialize>(&self, message: S, headers: Option<Headers>) -> hyper::error::Result<Response> {
        let body = ser::to_string(&message).expect("Could not serialize LoadNotification");

        // TODO: handle retry
        let response = Client::new()
              .post(self.url.clone())
              .headers(headers.unwrap_or(Headers::new()))
              .header(ContentType::json())
              .body(&body)
              .send();
        if let Err(ref err) = response {
            println!("Failed to send notification: {}", err);
        }
        response
    }
}

pub enum LoadStatus { Success, Failure(String) }

pub struct LoadNotification {
    slot_id: Option<String>,
    load: String,
    error: Option<String>,
    load_time: f64,
}

impl LoadNotification {
    pub fn new(load_status: LoadStatus, load_time: Duration) -> LoadNotification {
        let (load, error) = match load_status {
            LoadStatus::Success => ("Success", None),
            LoadStatus::Failure(err) => ("Failure", Some(err)),
        };
        LoadNotification {
            slot_id: env::var("SLOT_ID").ok(),
            load: load.to_owned(),
            error: error,
            load_time: load_time.num_microseconds().unwrap() as f64 / 1_000_000f64,
        }
    }
}

// JSON boilerplate - until compiler plugins are stable to just annotate with #[derive(Serialize)]
impl Serialize for LoadNotification {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        serializer.serialize_map(LoadNotificationMapVisitor{value: self})
    }
}
struct LoadNotificationMapVisitor<'a> { value: &'a LoadNotification }
impl<'a> serde::ser::MapVisitor for LoadNotificationMapVisitor<'a> {
    fn visit<S: Serializer>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> {
        try!(serializer.serialize_map_elt("slot_id", &self.value.slot_id));
        try!(serializer.serialize_map_elt("load", &self.value.load));
        try!(serializer.serialize_map_elt("load_time", &self.value.load_time));
        if let Some(ref error) = self.value.error {
            try!(serializer.serialize_map_elt("error", error));
        }
        Ok(None)
    }
}