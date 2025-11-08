use std::io;

pub fn reverse(input: &str) -> String
{
    input.chars().rev().collect()
}

fn main()
{
println!("Input a word to reverse");

let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read line");

let input = input.trim();
let output = reverse(input);

println!("{} is {}", input, output);
}
