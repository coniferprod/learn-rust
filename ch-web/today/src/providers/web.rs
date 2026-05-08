use std::str::FromStr;

use reqwest::blocking::{Client, Response};
use serde::Deserialize;
use serde_json;
use chrono::{NaiveDate, Datelike, Local};
use url::Url;

use crate::events::{Category, Event, MonthDay};
use crate::providers::EventProvider;

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

    fn get_events(&self, events: &mut Vec<Event>) {
        let today: NaiveDate = Local::now().date_naive();
        let month_day = MonthDay::new(today.month(), today.day());

        let mut url = self.url.clone();
        url.set_query(Some(&format!("date={}", month_day)));

        let client = Client::new();
        let request = client.get(url).send();
        let response: Response;
        if request.is_err() {
            eprintln!("Error while retrieving data: {:#?}", request.err());
            return;
        } else {
            response = request.ok().unwrap();
        }

        let json_events = response.json::<Vec<JSONEvent>>().unwrap();
        println!("Got {} events from JSON", json_events.len());
        for json_event in json_events {
            let date = NaiveDate::parse_from_str(&json_event.date, "%F").unwrap();
            let category = match Category::from_str(&json_event.category) {
                Ok(cat) => cat,
                Err(e) => {
                    eprintln!("{}", e);
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
