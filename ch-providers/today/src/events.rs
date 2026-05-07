//! Structs related to events.
//! 

use std::str::FromStr;
use std::fmt;

use chrono::{NaiveDate, Datelike};

/// Event kind, with associated value.
#[derive(Debug, Clone, PartialEq, Eq)]
enum EventKind {
    Singular(NaiveDate),
}

/// Represents a historical or observed event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    kind: EventKind,
    description: String,
    category: Category,
}

impl Event {
    /// Makes a new singular event with exact date, description, 
    /// and category.
    pub fn new_singular(date: NaiveDate, 
                        description: String, 
                        category: Category) -> Self {
        Self { 
            kind: EventKind::Singular(date),
            description, 
            category
        }
    }

    /// Gets the year of the event.
    pub fn year(&self) -> i32 {
        match &self.kind {
            EventKind::Singular(date) => date.year(),
        }
    }

    /// Gets the month-day of the event.
    pub fn month_day(&self) -> MonthDay {
        match &self.kind {
            EventKind::Singular(date) => 
                MonthDay { 
                    month: date.month(), 
                    day: date.day() 
                },
        }
    }

    /// Gets the description of the event.
    pub fn description(&self) -> String {
        self.description.clone()
    }

    /// Gets the category of the event.
    pub fn category(&self) -> Category {
        self.category.clone()
    }
}

/// Represents a month-day combination.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MonthDay {
    month: u32,
    day: u32,
}

impl MonthDay {
    /// Makes a new month-day. The values are not checked.
    pub fn new(month: u32, day: u32) -> Self {
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

impl fmt::Display for MonthDay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}-{:02}", self.month, self.day)
    }
}

/// Represents the category of the event,
/// with a mandatory primary part and an optional
/// secondary part.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Category {
    primary: String,
    secondary: Option<String>,
}

impl Category {
    /// Makes a new category with both primary and secondary parts.
    pub fn new(primary: &str, secondary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: Some(secondary.to_string()),
        }
    }

    /// Makes a new category with only the primary part.
    pub fn from_primary(primary: &str) -> Self {
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
