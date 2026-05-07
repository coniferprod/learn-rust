use std::error::Error;
use std::str::FromStr;

use chrono::{NaiveDate, Datelike};

mod birthday;
mod events;

use crate::events::{Event, Category};

pub fn run() -> Result<(), Box<dyn Error>> {
    birthday::handle_birthday();

    let mut events: Vec<Event> = Vec::new();

    let rust_category = match Category::from_str("programming/rust") {
        Ok(category) => category,
        Err(e) => {
            eprintln!("{e}");
            return Err(e.into());
        }
    };

    events.push(Event::new_singular(
        NaiveDate::from_ymd_opt(2025, 12, 11).unwrap(),
        String::from("Rust 1.92.0 released"),
        rust_category.clone()));

    events.push(Event::new_singular(
        NaiveDate::from_ymd_opt(2015, 5, 15).unwrap(),
        String::from("Rust 1.0.0 released"),
        rust_category.clone()));
        
    for event in events {
        println!("{}-{}: {} ({})", 
            event.year(), event.month_day(), 
            event.description(), 
            event.category());
    }

    Ok(())
}
