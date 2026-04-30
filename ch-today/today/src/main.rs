use std::str::FromStr;
use std::fmt;

use chrono::{NaiveDate, Datelike};

#[derive(Debug, Clone, PartialEq, Eq)]
enum EventKind {
    Singular(NaiveDate),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Event {
    kind: EventKind,
    description: String,
    category: Category,
}

impl Event {
    fn new_singular(date: NaiveDate, 
                        description: String, 
                        category: Category) -> Self {
        Self { 
            kind: EventKind::Singular(date),
            description, 
            category
        }
    }

    fn year(&self) -> i32 {
        match &self.kind {
            EventKind::Singular(date) => date.year(),
        }
    }

    fn month_day(&self) -> MonthDay {
        match &self.kind {
            EventKind::Singular(date) => 
                MonthDay { 
                    month: date.month(), 
                    day: date.day() 
                },
        }
    }    
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct MonthDay {
    month: u32,
    day: u32,
}

impl MonthDay {
    fn new(month: u32, day: u32) -> Self {
        Self { month, day }
    }
}

impl FromStr for MonthDay {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date_str = String::from(format!("2026{}", s));
        match NaiveDate::parse_from_str(&date_str, "%Y%m%d") {
            Ok(date) => Ok(MonthDay { month: date.month(), day: date.day() }),
            Err(e) => Err(format!("{}", e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
        if parts.len() > 1 { 
            Ok(Self::new(parts[0], parts[1]))
        } else {
            Ok(Self::from_primary(parts[0]))
        }
    }
}

fn main() {
    let mut events: Vec<Event> = Vec::new();

    let rust_category = match Category::from_str("programming/rust") {
        Ok(category) => category,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    events.push(Event::new_singular(
        NaiveDate::from_ymd_opt(2025, 12, 11).unwrap(),
        String::from("Rust 1.92.0 released"),
        rust_category.clone()));

    events.push(Event::new_singular(
        NaiveDate::from_ymd_opt(2015, 5, 15).unwrap(),
        String::from("Rust 1.0.0 released"),
        rust_category.clone()));
        
    for event in events {
        println!("{}: {} ({})", 
            event.year(), 
            event.description, 
            event.category);
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::MonthDay;

    #[test]
    fn month_day_from_str_accepts_valid() {
        assert_eq!(
            MonthDay::from_str("0515"), 
            Ok(MonthDay { month: 5, day: 15 })
        )
    }
}
