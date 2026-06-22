use std::path::{Path, PathBuf};

use log;
use url::Url;

use crate::events::Event;
use crate::filters::EventFilter;
use crate::Config;
use crate::providers::{
    EventProvider,
    textfile::TextFileProvider,
    csvfile::CSVFileProvider,
    sqlite::SQLiteProvider,
    web::WebProvider,
    xmlfile::XmlFileProvider,
};

pub struct EventManager {
    config_path: PathBuf,
    providers: Vec::<Box<dyn EventProvider>>,
}

pub struct ProviderInfo {
    pub name: String,
    pub kind: String,
    pub is_add_supported: bool,
    pub is_active: bool,
}

impl EventManager {
    pub fn new(config_path: &Path) -> Self {
        log::debug!("Making new EventManager, config_path = '{:?}'", 
            config_path.to_str());
        Self {
            config_path: config_path.to_path_buf(),
            providers: Vec::new(),
        }
    }

    pub fn create_providers(&mut self, config: &Config) {
        for cfg in config.providers.iter() {
            let found = self.providers.iter().any(|p| p.name() == cfg.name);
            if found {
                eprintln!("Event provider {} already exists", &cfg.name);
                continue;
            }

            let path = self.config_path.join(&cfg.resource);
            match cfg.kind.as_str() {
                "text" => {
                    let provider = TextFileProvider::new(
                        &cfg.name, 
                        &path, 
                        cfg.is_active.unwrap_or(true));
                    self.providers.push(Box::new(provider));
                },
                "csv" => {
                    let provider = CSVFileProvider::new(
                        &cfg.name, 
                        &path,
                        cfg.is_active.unwrap_or(true));
                    self.providers.push(Box::new(provider));
                },
                "sqlite" => {
                    let provider = SQLiteProvider::new(
                        &cfg.name, 
                        &path,
                        cfg.is_active.unwrap_or(true));
                    self.providers.push(Box::new(provider));
                },
                "web" => {
                    match Url::parse(&cfg.resource) {
                        Ok(url) => {
                            let provider = WebProvider::new(
                                &cfg.name, 
                                &url,
                                cfg.is_active.unwrap_or(true));
                            self.providers.push(Box::new(provider));
                        },
                        Err(e) => {
                            eprintln!("Error in URL for provider {}: {}",
                                &cfg.name, e);
                        }
                    }
                },
                "xml" => {
                    let provider = XmlFileProvider::new(
                        &cfg.name, 
                        &path,
                        cfg.is_active.unwrap_or(true));
                    self.providers.push(Box::new(provider));
                },
                _ => {
                    eprintln!("Unable to make provider: {:?}", cfg);
                }
            }
        }
    }

    pub fn get_provider_info(&self) -> Vec<ProviderInfo> {
        let mut result = Vec::new();

        for provider in &self.providers {
            result.push(ProviderInfo {
                name: provider.name(), 
                kind: provider.kind(),
                is_add_supported: provider.is_add_supported(),
                is_active: provider.is_active() });
        }

        result
    }

    /// Gets all events from all event providers that match the filter,
    /// and adds them to the supplied vector.
    pub fn get_events(&self, filter: &EventFilter) -> Vec<Event> {
        let mut result: Vec<Event> = Vec::new();

        let mut count = 0;

        for provider in &self.providers {
            if !provider.is_active() {     // OK, don't bother with this provider
                log::info!("Provider '{}' is not active, skipping",
                    provider.name());
                continue;
            }
            provider.get_events(&filter, &mut result);
            let new_count = result.len();
            log::info!(
                "Got {} events from provider '{}'", 
                new_count - count,
                provider.name());
            count = new_count;
        }

        result
    }

    pub fn add_event(&self, provider_name: &str, event: &Event) -> bool {
        // Find provider by name
        let mut provider: Option<&dyn EventProvider> = None;
        for p in &self.providers {
            if p.name() == provider_name {
                provider = Some(p.as_ref());
                break;
            }
        }

        match provider {
            Some(p) => {
                if p.is_active() {
                    if p.is_add_supported() {
                        let _ = p.add_event(event);
                        return true;
                    } else {
                        eprintln!("Adding events is not supported for provider '{}'", p.name());
                        return false;
                    }
                } else {
                    eprintln!("Provider '{}' is not active", p.name());
                    return false;
                }
            },
            None => {
                eprintln!("Unknown event provider '{}'", provider_name);
                return false;
            }
        }
    }
}
