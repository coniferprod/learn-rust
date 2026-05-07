fn main() {
    if let Err(e) = today::run() {
        eprintln!("Error");
    }
}

#[cfg(test)]
mod tests {
    use crate::MonthDay;
    use std::str::FromStr;

    #[test]
    fn month_day_from_str_accepts_valid() {
        assert_eq!(
            MonthDay::from_str("0515"),
            Ok(MonthDay { month: 5, day: 15 })
        )
    }
}
