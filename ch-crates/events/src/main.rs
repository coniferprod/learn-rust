use std::env;
use std::fmt;
use std::str::FromStr;

use rand::RngExt;

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

    fn is_valid(&self) -> bool {
        self.day >= 1 && self.day <= day_count(self.month, self.year)
    }
}

impl Date {
    fn random() -> Self {
        let mut rng = rand::rng();
        Self {
            year: rng.random_range(1900..2099),
            month: Month::from_i32(rng.random_range(1..=12)),
            day: rng.random_range(1..=31),
        }
    }
}

fn day_count(month: Month, year: i16) -> u8 {
    match month {
        Month::April | Month::June | Month::September | Month::November => 30,
        Month::February => if is_leap_year(year) { 29 } else { 28 },
        _ => 31
    }
}

fn is_leap_year(year: i16) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
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

    let dates = [
        // OK - 2000 was a leap year, so February has 29 days
        Date { year: 2000, month: Month::February, day: 29 },

        // bogus - April never has 31 days
        Date { year: 2026, month: Month::April, day: 31 },

        // OK - May has 31 days
        Date { year: 2012, month: Month::May, day: 31 },

        // bogus - day zero is never valid
        Date { year: 2026, month: Month::January, day: 0 },
    ];

    for date in dates {
        if date.is_valid() {
            println!("{:?} is valid", date);
        } else {
            println!("{:?} is bad", date);
        }
    }

    let mut random_dates = Vec::<Date>::new();
    let mut total_count = 0;
    let mut valid_count = 0;
    const MAX_COUNT: i32 = 100_000;
    while valid_count < MAX_COUNT {
        let candidate = Date::random();
        total_count += 1;
        if candidate.is_valid() {
            random_dates.push(candidate);
            valid_count += 1;
        }
    }
    println!("Generated {} random date candidates to get {} valid dates", 
        total_count, random_dates.len());    
}
