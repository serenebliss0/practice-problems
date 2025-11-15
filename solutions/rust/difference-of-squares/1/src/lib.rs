use std::io;

pub fn square_of_sum(n: u32) -> u32 {

    for i in 0..=n
    {
        let mut sum:u32 = 0;

        sum += i;
    }
    return n;
}

pub fn sum_of_squares(n: u32) -> u32 {
    
    for i in 0..=n
    {
        n.pow(2);
        let total = n + n;
        let total = n;
    }

    return n;

}

pub fn difference(n: u32) -> u32 
{

    let difference = square_of_sum(n) - sum_of_squares(n);
    return difference;
    
}

fn main()
{
    println!("Welcome user!");
    println!("Please enter a number!");

    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let number:u32 = number.trim().parse().expect("Enter a valid number");

    println!("Lets find the difference of sqaures of the number `{}`", number);
    
    let difference_of_squares = difference(number);
    println!("The difference of squares is {}!", difference_of_squares);
}
