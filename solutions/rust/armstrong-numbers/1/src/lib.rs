pub fn is_armstrong_number(num: u32) -> bool {

    let digits: Vec<u32> = num
    .to_string()
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .collect();

    let digits_length:u32 = digits.len() as u32;
    let mut  total = 0; //this will be compared to the original num

    let i:usize = 0;
    for i in i..digits.len()
    {
        let mut  x = digits[i].pow(digits_length);
        total += x;
    }

    if total == num
    {
        return true;
    }
    else
    {
        return false;
    };

}

fn main()
{
    println!("Welcome to Armstrong Numbers! ");
    println!("An Armstrong Number, also called a narcissistic number is a number that is the sum of its own digits raised to the power of the number of digits ");
    println!("To start, enter a number and see if its narcissistic or not!");

    let mut input_1 = String::new();
    std::io::stdin().read_line(&mut input_1).expect("Failed to read line");
    let user_number:u32 = input_1.trim().parse().expect("Type in a valid integer!");

    is_armstrong_number(user_number);

}