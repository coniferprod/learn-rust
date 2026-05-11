use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs;

use chrono::{Datelike, Local, NaiveDate};
use serde::Deserialize;
use url::Url;

use crate::events::{Category, Event, MonthDay};
use crate::providers::{
    EventProvider, 
    textfile::TextFileProvider,  // 0.26.0
    csvfile::CSVFileProvider,    // 0.27.0
    sqlite::SQLiteProvider,      // 0.30.0
    web::WebProvider,            // 0.31.0
}; 

mod birthday;
mod events;
mod providers;

pub fn run(config: &Config, config_path: &Path) -> Result<(), Box<dyn Error>> {
    birthday::handle_birthday();

    let mut events: Vec<Event> = Vec::new();

    let providers = create_providers(config, config_path);
    let mut count = 0;
    for provider in providers {
        provider.get_events(&mut events);  // polymorphism at work!
        let new_count = events.len();
        println!(
            "Got {} events from provider '{}'", 
            new_count - count,
            provider.name());
        count = new_count;
    }

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

#[derive(Deserialize, Debug)]
pub struct ProviderConfig {
    name: String,
    kind: String,
    resource: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    providers: Vec::<ProviderConfig>,
}

fn create_providers(config: &Config, config_path: &Path) -> Vec::<Box<dyn EventProvider>> {
    // Try to create all the event providers specified in `config`.
    // Put them in a vector of trait objects.
    let mut providers: Vec::<Box<dyn EventProvider>> = Vec::new();
    for cfg in config.providers.iter() {
        let found = providers.iter().any(|p| p.name() == cfg.name);
        if found {
            eprintln!("Event provider {} already exists", &cfg.name);
            continue;
        }

        let path = config_path.join(&cfg.resource);
        match cfg.kind.as_str() {
            "text" => {
                let provider = TextFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "csv" => {
                let provider = CSVFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "sqlite" => {
                let provider = SQLiteProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "web" => {
                match Url::parse(&cfg.resource) {
                    Ok(url) => {
                        let provider = WebProvider::new(&cfg.name, &url);
                        providers.push(Box::new(provider));
                    },
                    Err(e) => {
                        eprintln!("Error in URL for provider '{}': {}",
                            &cfg.name, e);
                    }
                }
            },
            _ => {
                eprintln!("Unknown provider kind in {:?}", cfg);
            }
        }
    }

    providers
}
