use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use crate::events::{Event, Category};
use crate::providers::EventProvider;

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

    fn get_events(&self, events: &mut Vec<Event>) {
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
                    match chrono::NaiveDate::parse_from_str(&date_string, "%F") {
                        Ok(date) => {
                            match Category::from_str(&category_string) {
                                Ok(category) => {
                                    let event = Event::new_singular(
                                        date,
                                        description.clone(),
                                        category);
                                    //println!("{:?}", &event);
                                    events.push(event);
                                },
                                Err(e) => {
                                    eprintln!("{}", e);
                                }
                            }
                        },
                        Err(_) => {
                            eprintln!("Invalid timestamp '{}'", date_string);
                        }
                    }
                    state = ReadingState::Date;
                },
            } // match state
        }
    }        
}
