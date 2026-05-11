use std::path::PathBuf;
use std::fs;
use today::{run, Config};

fn main() {
    const APP_NAME: &str = "today";
    let config_path = get_config_path(APP_NAME);
    match config_path { 
        Some(path) => {
            let toml_path = path.join(format!("{}.toml", APP_NAME));
            println!("Looking for configuration file '{}'", &toml_path.display());
            let config_str = fs::read_to_string(&toml_path)
                .expect("configuration file should exist");
            let config: Config = toml::from_str(&config_str)
                .expect("configuration file should be valid");
            println!("config: {:#?}", config);

            if let Err(e) = run(&config, &path) {
                eprintln!("Error running program: {}", e);
            }
        },
        None => {
            eprintln!("Unable to configure the application");
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
                eprintln!("Unable to create config directory for {}", app_name);
                return None;
            }
        }
        return Some(config_path);
    }
    None
}
