use std::path::{Path, PathBuf};
use std::str::FromStr;

use chrono::{NaiveDate, Local, Datelike};
use csv::ReaderBuilder;
use log;  // 0.35.0

use crate::EventProvider;
use crate::events::{Event, Category, Rule, MonthDay};
use crate::filters::EventFilter;

pub struct CSVFileProvider {
    name: String,
    path: PathBuf,
}

impl CSVFileProvider {
    pub fn new(name: &str, path: &Path) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf()
        }
    }
}

impl EventProvider for CSVFileProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        log::info!("Reading from {}", &self.path.display());

        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_path(self.path.clone()).expect("wanted an existing CSV file");

        for result in reader.records() {
            let record = result.unwrap();

            let mut date_string = record[0].to_string();
            let description = record[1].to_string();
            let category_string = record[2].to_string();

            // Check if the date string starts with a letter:
            let is_rule_based = date_string.chars().next().unwrap().is_alphabetic();
            if is_rule_based {
                let rule = Rule::parse(&date_string).unwrap();
                let category = Category::from_str(&category_string).unwrap();
                let event = Event::new_rule_based(rule, description, category);
                if filter.accepts(&event) {
                    events.push(event);
                }
                continue;
            }

            let is_yearless = date_string.starts_with("--");
            if is_yearless {
                let today: NaiveDate = Local::now().date_naive();
                let year_string = format!("{:04}-", today.year());
                date_string = date_string.replace("--", &year_string);
            }
            
            match NaiveDate::parse_from_str(&date_string, "%F") {
                Ok(date) => {
                    let category = Category::from_str(&category_string).unwrap();
                    let event: Event;
                    if is_yearless {
                        event = Event::new_annual(
                            MonthDay::new(date.month(), date.day()),
                            description.clone(), 
                            category);
                    } else {
                        event = Event::new_singular( 
                            date, 
                            description.clone(), 
                            category);
                    }
                    if filter.accepts(&event) {
                        events.push(event);
                    }
                },
                Err(_) => {
                    log::error!("Invalid date '{}'", date_string);
                }
            }
        }
    }
}
