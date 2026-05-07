use dirs;

use std::path::PathBuf;
use std::fs;

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

fn main() {
    match dirs::config_dir() {
        Some(dir) => println!("Config directory: '{}'", dir.display()),
        None => println!("No config directory found!")
    }

    const APP_NAME: &str = "today";
    let config_path = get_config_path(APP_NAME);
    match config_path {
        Some(path) => println!("Config path: '{}'", path.display()),
        None => println!("No config directory found!")
    }
}
