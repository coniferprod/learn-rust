use std::env;
use std::fmt;
use std::str::FromStr;

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

impl Default for Date {
    fn default() -> Self {
        Self { year: 1970, month: Month::January, day: 1 }
    }
}

#[derive(Debug, PartialEq)]
struct MonthDay {
    month: Month,
    day: u8,
}

impl FromStr for MonthDay {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(String::from("invalid length for string"));
        }

        let month_string = &s[..2];
        let month: Month;
        match month_string.parse() {
            Ok(m) => month = Month::from_i32(m),
            Err(_) => {
                return Err(String::from("invalid month"));
            }
        };

        let day: u8;
        match s[2..].parse() {
            Ok(d) => day = d,
            Err(_) => {
                return Err(String::from("invalid day"));
            }
        };

        Ok(MonthDay { month, day })
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

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.secondary {
            Some(sec) => write!(f, "{}/{}", self.primary, sec),
            None => write!(f, "{}", self.primary),
        }
    }
}

impl FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(String::from("empty category"));
        }

        let parts: Vec<&str> = s.split("/").collect();
        if parts.len() < 2 {
            Ok(Self { 
                primary: parts[0].to_string(), 
                secondary: None 
            })
        } else {
            Ok(Self { 
                primary: parts[0].to_string(), 
                secondary: Some(parts[1].to_string()) 
            })
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

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} ({})", 
            self.date.year, self.description, self.category)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: events mmdd");
        return;
    }

    let md_arg = &args[1];  // note: 1, not 0
    let month_day = match MonthDay::from_str(md_arg) {
        Ok(m) => m,
        Err(_) => {
            eprintln!("Bad month-day: {}", md_arg);
            return;
        }
    }; 
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
            // Now that there is a Display trait implementation for Event,
            // we can print it with one normal placeholder.
            println!("{event}");

            any_luck = true;
        }
    }

    if !any_luck {
        println!("No events for {:#?}", month_day);
    }

    let cat = Category::from_str("programming/rust").unwrap();
    assert_eq!(format!("{}", cat), "programming/rust");

    // Implementing the Display trait gets you an implementation of
    // the ToString trait for free:
    assert_eq!(cat.to_string(), "programming/rust");
}
