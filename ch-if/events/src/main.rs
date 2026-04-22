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
    let date_string = date.to_string();
    let month_string = &date_string[4..6];
    let month: i32 = month_string.parse().unwrap();
    let day_string = &date_string[6..8];
    let day: i32 = day_string.parse().unwrap();
    let month_day = (month, day);

    let mut any_luck = false;  // Boolean flag

    for event in events {
        // Get the month and day from the event's date:
        let event_date_string = event.0.to_string();
        let event_month_string = &event_date_string[4..6];
        let event_month: i32 = event_month_string.parse().unwrap();
        let event_day_string = &event_date_string[6..8];
        let event_day: i32 = event_day_string.parse().unwrap();
        let event_month_day = (event_month, event_day);

        // Get the event year separately for printing:
        let year_string = &event_date_string[..4];
        let year: i32 = year_string.parse().unwrap();

        // Compare the month-day pairs and print 
        // event year and description if equal:
        if event_month_day == month_day {
            println!("{}: {}", year, event.1);
            any_luck = true;
        }
    }

    if !any_luck {
        println!("No events for {date}");
    }
}
