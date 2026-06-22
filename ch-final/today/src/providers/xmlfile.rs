use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

use chrono::{NaiveDate, Local};
use xml::reader::{EventReader, XmlEvent};
use log;

use crate::events::{Event, EventDate, Category};
use crate::providers::{EventProvider, EventProviderError};
use crate::filters::EventFilter;

pub struct XmlFileProvider {
    name: String,
    path: PathBuf,
    is_active: bool,
}

impl XmlFileProvider {
    pub fn new(name: &str, path: &Path, is_active: bool) -> Self {
        Self { 
            name: name.to_string(), 
            path: path.to_path_buf(),
            is_active,
        }
    }
}

impl EventProvider for XmlFileProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        let file = File::open(self.path.clone()).expect("path to XML file");
        let file = BufReader::new(file);
        let parser = EventReader::new(file);

        // Set up some reusable event parts with default values.
        // When we encounter XML elements, we set these values with their content.
        let today: NaiveDate = Local::now().date_naive();
        let mut event_date = EventDate::Singular(today);

        // Holds the description of the pending event
        let mut pending_description: String = String::new();

        let mut event_category: Category = Category::from_primary("test");
        let mut pending_category_string = String::new();

        let mut new_events: Vec<Event> = Vec::new();

        let mut content = String::new();  // also reused inside the loop
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    content.clear();
                    match name.local_name.as_str() {
                        "events" => new_events.clear(),
                        "description" => pending_description.clear(),
                        "category" => pending_category_string.clear(),
                        _ => {}
                    }
                },
                Ok(XmlEvent::EndElement { name }) => {
                    log::debug!("end element {}", name);
                    match name.local_name.as_str() {
                        "events" => events.append(&mut new_events),
                        "event" => {
                            let event = Event::new(
                                event_date, 
                                pending_description.clone(), 
                                event_category.clone());
                            log::debug!("New event: {}", &event);
                            if filter.accepts(&event) {
                                new_events.push(event);
                            } else {
                                log::debug!("Filter rejects event");
                            }
                        },
                        "date" => {
                            log::debug!("content = '{}'", &content);
                            event_date = EventDate::parse(&content)
                                .expect("invalid date");
                            content.clear();
                        },
                        "description" => {
                            pending_description = content.clone();
                            content.clear();
                        },
                        "category" => {
                            log::debug!("pending_category_string = '{}'", 
                                &pending_category_string);
                            event_category = Category::from_str(&pending_category_string).unwrap();
                        },
                        "primary" => {
                            log::debug!("content = '{}'", &content);
                            pending_category_string.push_str(&content);
                        },
                        "secondary" => {
                            log::debug!("content = '{}'", &content);
                            pending_category_string.push('/');
                            pending_category_string.push_str(&content);
                        },
                        _ => ()
                    }
                },
                Ok(XmlEvent::Characters(text)) => {
                    log::debug!("characters: '{}'", text);
                    content.push_str(&text);
                },
                Err(e) => {
                    eprintln!("Error: {e}");
                    break;
                },
                _ => {},
            }
        }
    }

    fn is_add_supported(&self) -> bool { false }

    fn add_event(&self, _event: &Event) -> Result<(), EventProviderError> {
        Err(EventProviderError::OperationNotSupported)
    }

    fn kind(&self) -> String { String::from("XML") }

    fn is_active(&self) -> bool {
        self.is_active
    }
}
