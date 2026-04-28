#[derive(Debug, Copy, Clone)]
    struct Date {
    year: i16,
    month: u8,
    day: u8,
}

fn main() {
    let mut today = Date { year: 2026, month: 2, day: 11 };
    println!("today = {:?}", today);
    let tomorrow = &mut today;
    tomorrow.day += 1;
    println!("tomorrow = {:?}", today); // note what is being printed
}
