use time::{PrimitiveDateTime, Date, Time, Duration, Month};
use std::io;

pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    start + Duration::seconds(1_000_000_000)
}

fn main() {
    fn read<T: std::str::FromStr>(prompt: &str) -> T {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().ok().expect("Invalid input")
    }

    let year: i32 = read("Enter year:");
    let month_num: u8 = read("Enter month (1-12):");
    let day: u8 = read("Enter day (1-31):");
    let hour: u8 = read("Enter hour (0-23):");
    let minute: u8 = read("Enter minute (0-59):");
    let second: u8 = read("Enter second (0-59):");

    let month = match month_num {
        1 => Month::January,
        2 => Month::February,
        3 => Month::March,
        4 => Month::April,
        5 => Month::May,
        6 => Month::June,
        7 => Month::July,
        8 => Month::August,
        9 => Month::September,
        10 => Month::October,
        11 => Month::November,
        12 => Month::December,
        _ => panic!("Invalid month!"),
    };

    let start = PrimitiveDateTime::new(
        Date::from_calendar_date(year, month, day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap()
    );

    let gigasecond = after(start);

    println!("One billion seconds later: {}", gigasecond);
}
