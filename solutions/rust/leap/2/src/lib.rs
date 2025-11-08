use std::io;
pub fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 == 1) || year % 400 == 0
}

fn main()
{
    println!("Enter a year and see if its a leap year!");
    let mut  year =  String::new();
    io::stdin().read_line(&mut year).expect("Failed to read line");
    let year:u64 = year.trim().parse().expect("Enter a valid year");

    is_leap_year(year);
}