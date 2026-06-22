use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::io::BufWriter;
use std::fs::OpenOptions;

use chrono::{NaiveDate, Local, Datelike};
use csv::ReaderBuilder;
use log;  // 0.35.0

use crate::providers::{EventProvider, EventProviderError};
use crate::events::{Event, EventDate, Category, Rule, MonthDay};
use crate::filters::EventFilter;

pub struct CSVFileProvider {
    name: String,
    path: PathBuf,
    is_active: bool, // 1.3.0: new field
}

impl CSVFileProvider {
    pub fn new(name: &str, path: &Path, is_active: bool) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
            is_active
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
                let event = Event::new(EventDate::RuleBased(rule), description, category);
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
                    let event_date = if is_yearless {
                        let month_day = MonthDay::new(date.month(), date.day());
                        EventDate::Annual(month_day)
                    } else {
                        EventDate::Singular(date)
                    };
                    let event = Event::new(event_date, description, category);
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

    fn is_add_supported(&self) -> bool { true }  // override the default implementation

    fn add_event(&self, event: &Event) -> Result<(), EventProviderError> {
        if !self.is_add_supported() {
            return Err(EventProviderError::OperationNotSupported);
        }

        let file = OpenOptions::new()
            .append(true)
            .open(self.path.clone())
            .expect("path to text file for writing");

        let writer = BufWriter::new(file);
        let mut csv_writer = csv::Writer::from_writer(writer);

        let date_string = if event.is_singular() {
            format!("{}", event.date())
        } else {
            return Err(EventProviderError::OperationNotSupported);
        };

        csv_writer.write_record([
            date_string, 
            event.description(), 
            format!("{}", event.category())
        ]).unwrap();
        
        csv_writer.flush().unwrap();

        Ok(())
    }

    fn kind(&self) -> String {
        String::from("CSV")
    }

    fn is_active(&self) -> bool {  // 1.3.0: new method
        self.is_active  
    }    
}
