use crate::{Event, Category};
use chrono::{NaiveDate, Datelike, Local};

pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, events: &mut Vec<Event>);
}

/// A simple event provider that always inserts a test event
/// for today to the vector passed to the `get_events` method.
pub struct SimpleProvider {
    name: String,
}

impl SimpleProvider {
    /// Makes a new simple provider with the given name.
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl EventProvider for SimpleProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, events: &mut Vec<Event>) {
        let today: NaiveDate = Local::now().date_naive();

        let test_event = Event::new_singular(
            today, 
            String::from("Test event for today"), 
            Category::from_primary("test")
        );
        events.push(test_event);
    }
}
