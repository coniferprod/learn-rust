struct Date {
    year: i16,
    month: u8,
    day: u8,
}

impl Date {
    fn new(year: i16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }
}

#[derive(Debug, PartialEq)]
struct MonthDay {
    month: u8,
    day: u8,
}

struct Event {
    date: Date,
    description: String,
}

impl Event {
    fn new(date: Date, description: String) -> Self {
        Self { date, description }
    }

    fn month_day(&self) -> MonthDay {
        MonthDay { month: self.date.month, day: self.date.day }
    }
}

fn main() {
    let events = [
        Event::new(Date::new(2025, 12, 11), 
                   String::from("Rust 1.92.0 released")),
        Event::new(Date::new(1996, 1, 23), 
                   String::from("JDK 1.0 released")),
        Event::new(Date::new(2008, 12, 3),
                   String::from("Python 3.0 released")),
        Event::new(Date::new(2015, 5, 15),
                   String::from("Rust 1.0.0 released")),
        Event::new(Date::new(2025, 9, 16),
                   String::from("Java 25 released")),
        Event::new(Date::new(2025, 10, 7),
                   String::from("Python 3.14 released")),
        Event::new(Date::new(2025, 12, 11),
                   String::from("Rust 1.92.0 released")),
    ];

    let month_day = MonthDay { month: 5, day: 15 };

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
