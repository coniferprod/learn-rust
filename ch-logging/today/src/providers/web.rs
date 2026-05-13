use std::str::FromStr;

use reqwest::blocking::{Client, Response};
use serde::Deserialize;
use chrono::{NaiveDate, Datelike, Local};
use url::Url;
use log;  // 0.35.0

use crate::events::{Category, Event};
use crate::providers::EventProvider;
use crate::filters::EventFilter;

pub struct WebProvider {
    name: String,
    url: Url,
}

impl WebProvider {
    pub fn new(name: &str, url: &Url) -> Self {
        Self { 
            name: name.to_string(),
            url: url.clone()
        }
    }
}

impl EventProvider for WebProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        // We need a date query parameter for the URL, so if the filter
        // does not specify it, we are done.
        if filter.month_day().is_none() {
            log::error!("No month-day in filter");
            return;
        }

        let month_day = filter.month_day().unwrap();

        let mut url = self.url.clone();
        url.set_query(Some(&format!("date={}", month_day)));

        let client = Client::new();
        let request = client.get(url).send();
        let response: Response;
        if request.is_err() {
            log::error!("Error while retrieving data: {:#?}", request.err());
            return;
        } else {
            response = request.ok().unwrap();
        }

        let json_events = response.json::<Vec<JSONEvent>>().unwrap();
        log::info!("Got {} events from JSON", json_events.len());
        for json_event in json_events {
            let date = NaiveDate::parse_from_str(&json_event.date, "%F").unwrap();
            let category = match Category::from_str(&json_event.category) {
                Ok(cat) => cat,
                Err(e) => {
                    log::error!("{}", e);
                    continue;
                }
            };
            let event = Event::new_singular(date, json_event.description, category);
            events.push(event);
        }
    } 
}

#[derive(Deserialize, Debug)]
struct JSONEvent {
    category: String,
    date: String,
    description: String,
}
