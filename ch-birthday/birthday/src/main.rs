use std::env;

use chrono::{Local, NaiveDate, Datelike};

fn main() {
    const NAME: &str = "BIRTHDATE";
    let value = env::var(NAME);
    if value.is_err() {
        return;
    }
    let value = value.unwrap(); // we know it's there now
    println!("{} = {}", NAME, value);

    match NaiveDate::parse_from_str(&value, "%F") {
        Ok(birthdate) => {
            let mut result = String::new();

            let today: NaiveDate = Local::now().date_naive();
            if birthdate.month() == today.month() && birthdate.day() == today.day() {
                result.push_str("Happy birthday! ");
            }

            let diff = today.signed_duration_since(birthdate);
            let day_count = diff.num_days();
            result.push_str(&make_message(day_count));
            println!("{}", result);
        },
        Err(_) => {
            eprintln!("Error in {} environment variable: \
                '{}' is not a valid date.", NAME, value);
        }
    }    
}

fn make_message(day_count: i64) -> String {
    let mut message = String::new();

    if day_count > 0 {
        message.push_str(&format!("You are {} days old.", day_count));
        if day_count % 1000 == 0 {
            message.push_str(" That's a nice, round number!");
        }
    } else if day_count < 0 {
        message.push_str("Are you from the future?");
    } else {  // must be zero
        message.push_str("Looks like you're new here.");
    }

    message
}

#[cfg(test)]
mod tests {
    use crate::make_message;

    #[test]
    fn make_message_normal() {
        assert_eq!(
            make_message(12345_i64), 
            "You are 12345 days old.");
    }

    #[test]
    fn make_message_normal_nice() {
        assert_eq!(
            make_message(10000_i64), 
            "You are 10000 days old. That's a nice, round number!");
    }

    #[test]
    fn make_message_newborn() {
        assert_eq!(
            make_message(0),
            "Looks like you're new here.");
    }

    #[test]
    fn make_message_future() {
        assert_eq!(
            make_message(-1),
            "Are you from the future?");
    }
}    
