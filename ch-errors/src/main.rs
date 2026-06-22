use std::fs::File;
use std::io::{self, Read};

fn read_events(filename: &str) -> Result<String, io::Error> {
    let file_result = File::open(filename);

    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),  // propagate the error (1)
    };

    // OK, we have a file, let's read its content:

    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(e) => Err(e),  // propagate the error (2)
    }
}

fn main() {
    let filename = "events.txt";
    match read_events(&filename) {
        Ok(text) => {
            println!("{}", text);
        },
        Err(e) => {
            eprintln!("Unable to read from file '{}', error: {}", 
                filename, e);
        }
    }
}
