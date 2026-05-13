use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use chrono::{NaiveDate, Local, Datelike};

use crate::events::{Event, Category, MonthDay};
use crate::providers::EventProvider;
use crate::filters::EventFilter;

pub struct TextFileProvider {
    name: String,
    path: PathBuf,
}

impl TextFileProvider {
    pub fn new(name: &str, path: &Path) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf()
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
        println!("Reading from {}", &self.path.display());
        
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

                    let event: Event;
                    match NaiveDate::parse_from_str(&date_string, "%F") {
                        Ok(date) => {
                            let category = Category::from_str(&category_string).unwrap();
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
                            eprintln!("Invalid date '{}'", date_string);
                        }
                    }
                    state = ReadingState::Date;
                },
            } // match state
        }
    }        
}
