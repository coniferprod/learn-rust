use std::error::Error;

use serde::Deserialize;
use pluralizer::pluralize;

use crate::events::Event;
use crate::filters::EventFilter;  // 0.32.0
use log;  // 0.35.0
use crate::manager::EventManager;  // 1.3.0

pub mod birthday;
pub mod events;
pub mod providers;
pub mod filters;
pub mod manager; // 1.3.0

pub fn run_providers(manager: &EventManager) {
    log::debug!("run_providers");

    println!("! = active   + = supports add");
    let provider_info = manager.get_provider_info();
    for info in provider_info {
        println!(
            "{:20} {}{} {}", 
            info.name,
            if info.is_active { "!" } else { " "},
            if info.is_add_supported { "+" } else { " " },
            info.kind);
    }
}

pub fn run_add(manager: &EventManager, provider_name: &str, event: &Event) {
    log::debug!("run_add");

    if !manager.add_event(provider_name, event) {
        eprintln!("Unable to add event to provider '{}'", provider_name);
    }
}

pub fn run(manager: &EventManager, filter: &EventFilter)
        -> Result<(), Box<dyn Error>> {
    log::debug!("run");

    let events = manager.get_events(&filter);

    let (mut singular_events, annual_events): (Vec<&Event>, Vec<&Event>)
        = events.iter().partition(|event| event.is_singular());

    let include_count = true;
    let mut print_separator = true;

    if !singular_events.is_empty() {
        singular_events.sort_by(|a, b| a.year().cmp(&b.year()));
        singular_events.reverse();
        println!("On this day in history ({}):", 
            pluralize("event", singular_events.len() as isize, include_count));
        for event in singular_events {
            println!("{}", event);
        }
    } else {
        print_separator = false;
    }

    if !annual_events.is_empty() {
        if print_separator {
            println!();
        }
        println!("Observed today ({}):", 
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
    pub is_active: Option<bool>, // 1.3.0: new setting
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub providers: Vec::<ProviderConfig>,
}
