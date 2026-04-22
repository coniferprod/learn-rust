fn main() {
    let events = [
        (1996_01_23, "JDK 1.0 released"),
        (2008_12_03, "Python 3.0 released"),
        (2015_05_15, "Rust 1.0.0 released"),
        (2025_09_16, "Java 25 released"),
        (2025_10_07, "Python 3.14 released"),
        (2025_12_11, "Rust 1.92.0 released"),
    ];

    // With index number:
    /*
    for (index, event) in events.iter().enumerate() {
        println!("[{}] {}: {}", index + 1, event.0, event.1);
    }
    */

    for event in events {
        println!("{}: {}", event.0, event.1);
    }
}
