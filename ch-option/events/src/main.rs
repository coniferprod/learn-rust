#[derive(Debug)]
struct Category {
    primary: String,
    secondary: Option<String>,
}

impl Category {
    fn new(primary: &str, secondary: &str) -> Self {
        Self { 
            primary: primary.to_string(), 
            secondary: Some(secondary.to_string())
        }
    }

    fn from_primary(primary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: None
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Month {
    January, February, March,
    April, May, June,
    July, August, September,
    October, November, December,
}

struct Date {
    year: i16,
    month: Month,
    day: u8,
}

impl Date {
    fn new(year: i16, month: Month, day: u8) -> Self {
        Self { year, month, day }
    }
}

#[derive(Debug, PartialEq)]
struct MonthDay {
    month: Month,
    day: u8,
}

struct Event {
    date: Date,
    description: String,
    category: Category,
}

impl Event {
    fn new(date: Date, description: String, category: Category) -> Self {
        Event { date, description, category }
    }
}

fn main() {
    let events = [
        Event::new(
            Date::new(2025, Month::December, 11), 
            String::from("Rust 1.92.0 released"),
            Category::new("programming", "rust")),
        Event::new(
            Date::new(1996, Month::January, 23), 
            String::from("JDK 1.0 released"),
            Category::new("programming", "java")),
        Event::new(
            Date::new(2008, Month::December, 3),
            String::from("Python 3.0 released"),
            Category::new("programming", "python")),
        Event::new(
            Date::new(2015, Month::May, 15),
            String::from("Rust 1.0.0 released"),
            Category::new("programming", "rust")),
        Event::new(
            Date::new(2025, Month::September, 16),
            String::from("Java 25 released"),
            Category::new("programming", "java")),
        Event::new(
            Date::new(2025, Month::October, 7),
            String::from("Python 3.14 released"),
            Category::new("programming", "python")),
        Event::new(
            Date::new(2025, Month::December, 11),
            String::from("Rust 1.92.0 released"),
            Category::new("programming", "rust")),
    ];

    for event in events {
        println!(
            "{}: {} ({:#?})", 
            event.date.year, 
            event.description,
            event.category);
    }
}
