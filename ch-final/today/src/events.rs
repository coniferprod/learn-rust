//! Structs related to events.
//!

use std::fmt;
use std::str::FromStr;

use strum_macros;
use chrono::{
    NaiveDate, 
    Datelike, 
    Local, 
    Month,  // needed for the Rule in the RuleBased enum variant 
    Weekday as ChronoWeekday,
};
use log;

/// Event kind, with associated value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventKind {
    Singular(NaiveDate),
    Annual(MonthDay),
    RuleBased(Rule),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rule {
    ordinal: Ordinal,
    weekday: Weekday,  // not chrono::Weekday!
    month: Month,
}

impl Rule {
    // Parse a rule of the following format:
    // first|second|third|fourth|fifth|last <weekday> in <month>
    //   weekday: Monday|Tuesday|Wednesday|Thursday|Friday|Saturday|Sunday
    //   month: January|February|March| ... |November|December
    pub fn parse(rule_string: &str) -> Option<Self> {
        let parts: Vec<String> = rule_string.to_lowercase().split_whitespace()
            .map(str::to_string).collect();

        // After splitting on whitespace, there must be exactly four parts.
        if parts.len() != 4 {
            log::error!("invalid rule: {}", rule_string);
            return None;
        }

        let ordinal = match Ordinal::from_str(&parts[0]) {
            Ok(ord) => ord,
            Err(e) => {
                log::error!("{}", e);
                return None;
            }
        };

        let weekday = match Weekday::from_str(&parts[1]) {
            Ok(wd) => wd,
            Err(e) => {
                log::error!("{}", e);
                return None;
            }
        };

        if parts[2] != "in" && parts[2] != "of" {
            log::error!("rule should specify `in` or `of`");
            return None;
        }

        let month = match parts[3].parse::<Month>() {
            Ok(m) => m,
            Err(e) => {
                log::error!("{}", e);
                return None;
            }
        };

        Some(Self { ordinal, weekday, month })
    }

    pub fn year(&self) -> i32 {
        Local::now().year()  // always use the year from the current local date
    }

    pub fn month_day(&self) -> Option<MonthDay> {
        if let Some(date) = self.resolve_date(self.year()) {
            Some(MonthDay { month: date.month(), day: date.day() })
        } else {
            None
        }
    }

    pub fn resolve_date(&self, year: i32) -> Option<NaiveDate> {
        if self.ordinal == Ordinal::Last {
            last_weekday_in_month(year, self.month, self.weekday)
        } else {
            nth_weekday_in_month(year, self.month, self.weekday, self.ordinal)
        }
    }    
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} in {}",
            self.ordinal, self.weekday, self.month.name())
    }
}

fn last_weekday_in_month(year: i32, 
                         month: Month, 
                         weekday: Weekday) -> Option<NaiveDate> {
    for day in (1..=31).rev() {   // note that the range is reversed!
        if let Some(date) = NaiveDate::from_ymd_opt(year, month.number_from_month(), day) {
            if date.weekday() == weekday.as_chrono_weekday() {
                return Some(date);
            }
        }
    }

    None
}

fn nth_weekday_in_month(year: i32, 
                        month: Month, 
                        weekday: Weekday, 
                        ordinal: Ordinal) -> Option<NaiveDate> {
    let mut count = 0;

    for day in 1..=31 {
        if let Some(date) = NaiveDate::from_ymd_opt(year, month.number_from_month(), day) {
            if date.weekday() == weekday.as_chrono_weekday() {
                count += 1;
                if count == ordinal as i32 {
                    return Some(date);
                }
            }
        }
    }

    None
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, 
    strum_macros::EnumString, strum_macros::Display)]
#[strum(ascii_case_insensitive)]
enum Ordinal {
    First = 1,
    Second = 2,
    Third = 3,
    Fourth = 4,
    Fifth = 5,
    Last = 6,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, 
    strum_macros::EnumString, strum_macros::Display)]
#[strum(ascii_case_insensitive)]
pub enum Weekday {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6,
}

impl Weekday {
    pub fn as_chrono_weekday(&self) -> ChronoWeekday {
        match *self {
            Weekday::Monday => ChronoWeekday::Mon,
            Weekday::Tuesday => ChronoWeekday::Tue,
            Weekday::Wednesday => ChronoWeekday::Wed,
            Weekday::Thursday => ChronoWeekday::Thu,
            Weekday::Friday => ChronoWeekday::Fri,
            Weekday::Saturday => ChronoWeekday::Sat,
            Weekday::Sunday => ChronoWeekday::Sun,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EventDate {
    Singular(NaiveDate),
    Annual(MonthDay),
    RuleBased(Rule),
}

impl EventDate {
    pub fn parse(date_string: &str) -> Option<EventDate> {
        if date_string.is_empty() {
            log::warn!("empty date string");
            return None;
        }

        let is_rule_based = date_string.chars().next().unwrap().is_alphabetic();
        if is_rule_based {
            match Rule::parse(&date_string) {
                Some(rule) => return Some(EventDate::RuleBased(rule)),
                None => {
                    log::error!("error parsing rule");
                    return None;
                }
            }
        }
        log::debug!("not rule-based");

        let is_yearless = date_string.starts_with("--");
        if is_yearless {
            let md_string: String = date_string.chars()
                .filter(|c| *c != '-')
                .collect();
            log::debug!("md_string = '{}'", &md_string);
            match MonthDay::from_str(&md_string) {
                Ok(month_day) => return Some(EventDate::Annual(month_day)),
                Err(e) => {
                    log::error!("error parsing yearless date: {}", e);
                    return None;
                }
            }
        } else {
            log::debug!("not yearless");

            match NaiveDate::parse_from_str(&date_string, "%F") {
                Ok(date) => return Some(EventDate::Singular(date)),
                Err(_) => {
                    log::error!("Invalid date '{}'", date_string);
                    return None;
                }
            }
        }
    }
}

impl fmt::Display for EventDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventDate::Singular(date) => write!(f, "{}", date),
            EventDate::Annual(month_day) => write!(f, "--{}", month_day),
            EventDate::RuleBased(rule) => write!(f, "{}", rule),
        }
    }
}

/// Represents a historical or observed event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    date: EventDate,
    description: String,
    category: Category,
}

impl Event {
    /// Makes a new event with date, description, and category.
    pub fn new(date: EventDate, description: String, category: Category) -> Self {
        Self {
            date,
            description,
            category
        }
    }

    /// Gets the year of the event. For annual events, it is always
    /// the ongoing year.
    pub fn year(&self) -> i32 {
        match &self.date {
            EventDate::Singular(date) => date.year(),
            EventDate::Annual(_month_day) => {
                let today: NaiveDate = Local::now().date_naive();
                today.year()
            },
            EventDate::RuleBased(rule) => rule.year(),
        }
    }

    /// Gets the month-day of the event.
    pub fn month_day(&self) -> MonthDay {
        match &self.date {
            EventDate::Singular(date) => MonthDay::new(date.month(), date.day()),
            EventDate::Annual(month_day) => month_day.clone(),
            EventDate::RuleBased(rule) => {
                match rule.month_day() {
                    Some(month_day) => month_day,
                    None => panic!("invalid month day resolved from rule"),
                }
            }
        }
    } 

    pub fn date(&self) -> EventDate {
        self.date.clone()
    }

    /// Returns `true` if the event is singular, `false` otherwise.
    pub fn is_singular(&self) -> bool {
        match self.date {
            EventDate::Singular(_) => true,
            _ => false
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

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} ({})",
            self.year(),
            self.description,
            self.category)
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
            Ok(date) => Ok(MonthDay {
                month: date.month(),
                day: date.day(),
            }),
            Err(e) => Err(format!("{}", e)),
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

    pub fn primary(&self) -> String {
        self.primary.clone()
    }

    pub fn secondary(&self) -> Option<String> {
        self.secondary.clone()
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn rejects_invalid_ordinal() {
        assert_eq!(Rule::parse("sixth sunday in may"), None);
    }

    #[test]
    fn rejects_invalid_weekday() {
        assert_eq!(Rule::parse("first bloomsday in june"), None);
    }

    #[test]
    fn rejects_invalid_month() {
        assert_eq!(Rule::parse("first tuesday in remember"), None);
    }

    #[test]
    fn valid_date_from_rule() {
        let rule = Rule::parse("second tuesday of may").unwrap();
        assert_eq!(
            Some(rule.resolve_date(2026)), 
            Some(NaiveDate::from_ymd_opt(2026, 5, 12)));
    }

    #[test]
    fn last_weekday() {
        // last Monday in January 2026 should be 2026-01-26
        let target = NaiveDate::from_ymd_opt(2026, 1, 26);
        assert_eq!(
            Some(last_weekday_in_month(2026, Month::January, Weekday::Monday)),
            Some(target));
    }

    #[test]
    fn first_weekday() {
        // first Friday in May 2026 should be 2026-05-01
        let target = NaiveDate::from_ymd_opt(2026, 5, 1);
        assert_eq!(
            Some(nth_weekday_in_month(2026, Month::May, Weekday::Friday, Ordinal::First)),
            Some(target));
    }

    #[test]
    fn ordinal_display_is_correct() {
        assert_eq!(format!("{}", Ordinal::Last), "Last");
    }

    #[test]
    fn weekday_display_is_correct() {
        assert_eq!(format!("{}", Weekday::Sunday), "Sunday");
    }

    #[test]
    fn rule_based_event_date_parsed_correctly() {
        assert_eq!(
            EventDate::parse("first Monday in June"),
            Some(EventDate::RuleBased(Rule { 
                ordinal: Ordinal::First, 
                weekday: Weekday::Monday, 
                month: Month::June})));
    }

    #[test]
    fn yearless_event_date_parsed_correctly() {
        assert_eq!(
            EventDate::parse("--01-01"),
            Some(EventDate::Annual(MonthDay { month: 1, day: 1 }))
        )
    }
}
