use std::error::Error;

use chrono::{NaiveDate, Datelike, Local};

use crate::events::{Event, Category, MonthDay};
use crate::providers::{EventProvider, SimpleProvider};  // 0.25.0

mod birthday;
mod events;
mod providers;  // new in 0.25.0: event providers

pub fn run() -> Result<(), Box<dyn Error>> {
    birthday::handle_birthday();

    let mut events: Vec<Event> = Vec::new();

    // 0.25.0:
    // Create an instance of our simple event provider
    // and use it to get any events it has to provide:
    let test_provider = SimpleProvider::new("test");
    test_provider.get_events(&mut events);

    let today: NaiveDate = Local::now().date_naive();
    let today_month_day = MonthDay::new(today.month(), today.day());

    for event in events {
        if today_month_day == event.month_day() {
            println!("{}: {} ({})", 
                event.year(),
                event.description(),
                event.category());
        }
    }

    Ok(())
}
