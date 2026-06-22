use std::path::{Path, PathBuf};
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::str::FromStr;
use std::fs::{File, OpenOptions};

use chrono::{NaiveDate, Local, Datelike};
use log;  // 0.35.0

use crate::events::{Event, EventDate, Category, MonthDay};
use crate::providers::{EventProvider, EventProviderError};
use crate::filters::EventFilter;

pub struct TextFileProvider {
    name: String,
    path: PathBuf,
    is_active: bool,
}

impl TextFileProvider {
    pub fn new(name: &str, path: &Path, is_active: bool) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
            is_active,
        }
    }
}

enum ReadingState {
    Date,
    Description,
    Category,
    Separator,
}

impl EventProvider for TextFileProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        log::info!("Reading from {}", &self.path.display());
        
        let f = File::open(self.path.clone()).expect("path to text file");
        let reader = BufReader::new(f);
        let mut state = ReadingState::Date;
        let mut date_string = String::new();
        let mut description = String::new();
        let mut category_string = String::new();

        for line_result in reader.lines() {
            let line = line_result.expect("wanted to read a line");
            match state {
                ReadingState::Date => {
                    date_string = line;
                    state = ReadingState::Description;
                },
                ReadingState::Description => {
                    description = line;
                    state = ReadingState::Category;
                },
                ReadingState::Category => {
                    category_string = line;
                    state = ReadingState::Separator;
                },
                ReadingState::Separator => {
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
                            let event = Event::new(event_date, description.clone(), category);
                            if filter.accepts(&event) {
                                events.push(event);
                            }
                        },
                        Err(_) => {
                            log::error!("Invalid date '{}'", date_string);
                        }
                    }
                    state = ReadingState::Date;
                },
            } // match state
        }
    }

    fn is_add_supported(&self) -> bool { true }

    fn add_event(&self, event: &Event) -> Result<(), EventProviderError> {
        if !self.is_add_supported() {
            return Err(EventProviderError::OperationNotSupported);
        }

        let file = OpenOptions::new()
            .append(true)
            .open(self.path.clone())
            .expect("path to text file for writing");

        let mut writer = BufWriter::new(file);

        return if event.is_singular() {
            let _ = writeln!(writer, "{}", event.date());
            let _ = writeln!(writer, "{}", event.description());
            let _ = writeln!(writer, "{}", event.category());
            let _ = writeln!(writer, "");
            Ok(())
        } else {
            Err(EventProviderError::OperationNotSupported)
        };
    }

    fn kind(&self) -> String {
        String::from("text")
    }

    fn is_active(&self) -> bool {
        self.is_active
    }
}
