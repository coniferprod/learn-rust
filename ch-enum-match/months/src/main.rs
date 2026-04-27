#[derive(Debug, PartialEq, Copy, Clone)]
enum Month {
    January, February, March,
    April, May, June,
    July, August, September,
    October, November, December,
}

impl Month {
    fn day_count(&self, year: i32) -> u8 {
        match self {
            Month::April | Month::June | Month::September | Month::November => 30,
            Month::February => if is_leap_year(year) { 29 } else { 28 },
            _ => 31
        }
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn main() {
    let year = 2020;
    let month = Month::February;
    let count = month.day_count(year);

    println!("In {year}, February had {count} days.");
}
