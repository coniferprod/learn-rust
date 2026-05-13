use std::error::Error;
use std::path::Path;

use chrono::{Datelike, Local, NaiveDate};
use serde::Deserialize;
use url::Url;

use crate::events::Event;
use crate::providers::{
    EventProvider, 
    textfile::TextFileProvider,  // 0.26.0
    csvfile::CSVFileProvider,    // 0.27.0
    sqlite::SQLiteProvider,      // 0.30.0
    web::WebProvider,            // 0.31.0
};
use crate::filters::EventFilter;  // 0.32.0

mod birthday;
pub mod events;
mod providers;
pub mod filters;

pub fn run(config: &Config, config_path: &Path, filter: &EventFilter) -> Result<(), Box<dyn Error>> {
    birthday::handle_birthday();

    let providers = create_providers(config, config_path);
    let mut count = 0;
    let mut events: Vec<Event> = Vec::new();
    for provider in providers {
        provider.get_events(&filter, &mut events);
        let new_count = events.len();
        println!(
            "Got {} events from provider '{}'", 
            new_count - count,
            provider.name());
        count = new_count;
    }

    for event in events {
        println!("{}: {} ({})",
            event.year(),
            event.description(),
            event.category());
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct ProviderConfig {
    pub name: String,
    pub kind: String,
    pub resource: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub providers: Vec::<ProviderConfig>,
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
