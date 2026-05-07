use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs;

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

    let mut text_file_path = PathBuf::new();
    let mut csv_file_path = PathBuf::new();
    const APP_NAME: &str = "today";
    if let Some(path) = get_config_path(APP_NAME) {
        text_file_path.push(&path);
        csv_file_path.push(&path);
    };
    text_file_path.push("compsci.txt");
    csv_file_path.push("compsci.csv");

    // Create an instance of the new TextFileProvider
    // and use it to get any events it has to provide.
    // We supply both the name and the path to the text file.
    let text_file_provider = TextFileProvider::new(
        "compsci", &text_file_path);
    text_file_provider.get_events(&mut events);

    // Create an instance of the new CSVFileProvider
    // and use it to get any events it has to provide.
    // We supply both the name and the path to the text file.
    let csv_file_provider = CSVFileProvider::new(
        "compsci", &csv_file_path);
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

// Gets the configuration directory path for `app_name`.
// If the directory does not exist, tries to create it.
// Returns an optional `PathBuf` containing the directory path,
// or `None` if the directory can't be created.
fn get_config_path(app_name: &str) -> Option<PathBuf> {
    if let Some(config_dir) = dirs::config_dir() {
        let config_path = config_dir.join(app_name);
        if !config_path.exists() {
            if let Err(_) = fs::create_dir(&config_path) {
                eprintln!("Unable to create config directory for {}", app_name);
                return None;
            }
        }
        return Some(config_path);
    }
    None
}
