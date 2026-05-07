use std::error::Error;
use std::path::Path;

use chrono::{Datelike, Local, NaiveDate};

use crate::events::{Category, Event, MonthDay};
use crate::providers::{
    EventProvider, 
    textfile::TextFileProvider,  // 0.26.0
    csvfile::CSVFileProvider,    // 0.27.0
}; 

mod birthday;
mod events;
mod providers;

pub fn run() -> Result<(), Box<dyn Error>> {
    birthday::handle_birthday();

    let mut events: Vec<Event> = Vec::new();

    // Create an instance of the new TextFileProvider
    // and use it to get any events it has to provide.
    // We supply both the name and the path to the text file.
    let text_file_provider = TextFileProvider::new(
        "compsci", Path::new("compsci.txt"));
    text_file_provider.get_events(&mut events);

    // Create an instance of the new CSVFileProvider
    // and use it to get any events it has to provide.
    // We supply both the name and the path to the text file.
    let csv_file_provider = CSVFileProvider::new(
        "compsci", Path::new("compsci.csv"));
    csv_file_provider.get_events(&mut events);

    // Now the events from both text file and CSV file
    // should be in the same vector `events`.

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
