use std::fs::File;

fn main() {
    let file_result = File::open("events.txt");

    let file = match file_result {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {e}");
            return;
        }
    };

    println!("File was found, OK to start reading.");
}
