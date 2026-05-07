use dirs;

fn main() {
    match dirs::config_dir() {
        Some(dir) => println!("Config directory: '{}'", dir.display()),
        None => println!("No config directory found!")
    }
}
