use std::fmt;
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

impl Default for Date {
    fn default() -> Self {
        Self {
            year: 1970,
            month: 1,
            day: 1,
        }
    }
}
impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
fn main() {
    let epoch: Date = Default::default();
    let rollover = Date::new(2038, 1, 19);
    println!(
        "Unix time started on {} and will end on {}. Well, not anymore.",
        epoch, rollover
    );
}
