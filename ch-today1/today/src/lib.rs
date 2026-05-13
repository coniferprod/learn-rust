use std::error::Error;
use std::path::Path;

use chrono::{Datelike, Local, NaiveDate};
use serde::Deserialize;
use url::Url;
use pluralizer::pluralize;

use crate::events::{Event, EventKind};
use crate::providers::{
    EventProvider, 
    textfile::TextFileProvider,  // 0.26.0
    csvfile::CSVFileProvider,    // 0.27.0
    sqlite::SQLiteProvider,      // 0.30.0
    web::WebProvider,            // 0.31.0
};
use crate::filters::EventFilter;  // 0.32.0
use log;  // 0.35.0

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
        log::info!(
            "Got {} events from provider '{}'", 
            new_count - count,
            provider.name());
        count = new_count;
    }

    let (mut singular_events, annual_events): (Vec<&Event>, Vec<&Event>)
        = events.iter().partition(|event| match event.kind() {
            EventKind::Singular(_) => true,
            _ => false
        });

    let include_count = true;

    if !singular_events.is_empty() {
        singular_events.sort_by(|a, b| a.year().cmp(&b.year()));
        singular_events.reverse();
        println!("On this day in history ({}):", 
            pluralize("event", singular_events.len() as isize, include_count));
        for event in singular_events {
            println!("{}", event);
        }
    }

    if !annual_events.is_empty() {
        println!("\nObserved today ({}):", 
            pluralize("event", annual_events.len() as isize, include_count));
        for event in annual_events {
            println!("{} ({})", event.description(), event.category());
        }
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

pub fn create_providers(config: &Config, config_path: &Path) -> Vec::<Box<dyn EventProvider>> {
    // Try to create all the event providers specified in `config`.
    // Put them in a vector of trait objects.
    let mut providers: Vec::<Box<dyn EventProvider>> = Vec::new();
    for cfg in config.providers.iter() {
        let found = providers.iter().any(|p| p.name() == cfg.name);
        if found {
            log::error!("Event provider {} already exists", &cfg.name);
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
                        log::error!("Error in URL for provider '{}': {}",
                            &cfg.name, e);
                    }
                }
            },
            _ => {
                log::error!("Unknown provider kind in {:?}", cfg);
            }
        }
    }

    providers
}

pub fn add_event(config: &Config, config_path: &Path, provider_name: &str, event: &Event) {
    let providers = create_providers(config, config_path);

    // Find provider by name
    let mut provider: Option<&dyn EventProvider> = None;
    for p in &providers {
        if p.name() == provider_name {
            provider = Some(p.as_ref());
            break;
        }
    }

    match provider {
        Some(p) => {
            if p.is_add_supported() {
                let _ = p.add_event(event);
            } else {
                println!("Adding events is not supported for provider '{}'", p.name());
            }
        },
        None => {
            eprintln!("Unknown event provider '{}'", provider_name);
        }
    }
}
