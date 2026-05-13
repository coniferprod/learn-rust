use std::path::PathBuf;
use std::fs;
use std::str::FromStr;

use clap::{Parser, Subcommand};  // 0.34.0
use chrono::{NaiveDate, Local, Datelike};
use log;  // 0.35.0
use today::events::MonthDay;
use today::filters::{EventFilter, FilterBuilder};
use today::{run, Config};

#[derive(Subcommand, Debug, Clone)]
enum Command {
    /// List all event providers
    Providers,
}

#[derive(Parser)]
#[command(name = "today")]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,

    #[arg(short, long, help = "Event date in MMDD format")]
    date: Option<String>,  // optional, defaults to current date
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    let month_day = if let Some(md) = args.date {
        MonthDay::from_str(&md).unwrap()
    } else { 
        let today: NaiveDate = Local::now().date_naive();
        MonthDay::new(today.month(), today.day())
    };
    log::debug!("month_day = {:#?}", month_day);

    let filter: EventFilter = FilterBuilder::new()
        .month_day(month_day)
        .build();

    const APP_NAME: &str = "today";
    let config_path = get_config_path(APP_NAME);
    match config_path { 
        Some(path) => {
            let toml_path = path.join(format!("{}.toml", APP_NAME));
            log::info!("Looking for configuration file '{}'", &toml_path.display());
            let config_str = fs::read_to_string(&toml_path)
                .expect("configuration file should exist");
            let config: Config = toml::from_str(&config_str)
                .expect("configuration file should be valid");
            log::debug!("config: {:#?}", config);
            match args.cmd {
                Some(Command::Providers) => {
                    for provider in config.providers {
                        println!("{}", provider.name);
                    }
                },

                _ => {
                    if let Err(e) = run(&config, &path, &filter) {
                        log::error!("Error running program: {}", e);
                        return;
                    }
                }
            }
        },
        None => {
            log::error!("Unable to configure the application");
            return;
        }
    }
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
                log::error!("Unable to create config directory for {}", app_name);
                return None;
            }
        }
        return Some(config_path);
    }
    None
}
