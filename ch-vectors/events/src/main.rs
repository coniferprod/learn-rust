#[derive(Debug, PartialEq, Copy, Clone)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Debug, Copy, Clone)]
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

struct Event {
    date: Date,
    description: String,
    category: Category,
}

impl Event {
    fn new(date: Date, description: String, category: Category) -> Self {
        Self { date, description, category }
    }

    fn month_day(&self) -> MonthDay {
        MonthDay {
            month: self.date.month,
            day: self.date.day,
        }
    }
}

fn main() {
    let events = vec![
        Event::new(
            Date::new(2025, Month::December, 11),
            String::from("Rust 1.92.0 released"),
            Category::new("programming", "rust"),
        ),
        Event::new(
            Date::new(1996, Month::January, 23),
            String::from("JDK 1.0 released"),
            Category::new("programming", "java"),
        ),
        Event::new(
            Date::new(2008, Month::December, 3),
            String::from("Python 3.0 released"),
            Category::new("programming", "python"),
        ),
    ];

    let month_day = MonthDay { month: Month::December, day: 11 };

    let mut any_luck = false;  // Boolean flag

    for event in events {
        if event.month_day() == month_day {
            println!("{}: {}", event.date.year, event.description);
            any_luck = true;
        }
    }

    if !any_luck {
        println!("No events for {:#?}", month_day);
    }
}
