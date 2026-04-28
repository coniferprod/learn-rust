use std::env;

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

impl Month {
    fn from_i32(number: i32) -> Self {
        match number {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => panic!("Invalid month number: {}", number),
        }
    }
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

impl MonthDay {
    fn from_str(s: &str) -> Self {
        assert!(s.len() == 4);
        let month_string = &s[..2];
        let month = Month::from_i32(month_string.parse().unwrap());
        let day: u8 = s[2..].parse().unwrap();
        MonthDay { month, day }
    }
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
            secondary: Some(secondary.to_string()),
        }
    }

    fn from_primary(primary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: None,
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
        Self {
            date,
            description,
            category,
        }
    }

    fn month_day(&self) -> MonthDay {
        MonthDay {
            month: self.date.month,
            day: self.date.day,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let month_day = MonthDay::from_str(&args[1]); // note: 1, not 0
    //println!("{:#?}", month_day);

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

    let mut any_luck = false; // Boolean flag

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
