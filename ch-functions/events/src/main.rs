fn get_year_month_day(date: i32) -> (i32, i32, i32) {
    let date_string = date.to_string();
    assert!(date_string.len() == 8);

    let year_string = &date_string[..4];
    let year = year_string.parse().unwrap();

    let month_string = &date_string[4..6];
    let month = month_string.parse().unwrap();

    let day_string = &date_string[6..8];
    let day = day_string.parse().unwrap();

    (year, month, day)
}

fn main() {
    let events = [
        (1996_01_23, "JDK 1.0 released"),
        (2008_12_03, "Python 3.0 released"),
        (2015_05_15, "Rust 1.0.0 released"),
        (2025_09_16, "Java 25 released"),
        (2025_10_07, "Python 3.14 released"),
        (2025_12_11, "Rust 1.92.0 released"),
    ];

    // Get the month and day from our target date:
    let date = 20150515;
    let date_parts = get_year_month_day(date);
    let month_day = (date_parts.1, date_parts.2);

    let mut any_luck = false;  // Boolean flag

    for event in events {
        // Get the month and day from the event's date:
        let event_date_parts = get_year_month_day(event.0);

        // Make the month-day pairs, compare them, and print 
        // event year and description if they are equal:        
        let event_month_day = (event_date_parts.1, event_date_parts.2);
        if event_month_day == month_day {
            println!("{}: {}", event_date_parts.0, event.1);
            any_luck = true;
        }
    }

    if !any_luck {
        println!("No events for {date}");
    }
}
