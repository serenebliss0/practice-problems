pub fn raindrops(n: u32) -> String {

    let mut result = String::new();

    if n % 3 ==0
    {
        result.push_str("Pling");
    }
    if n % 5 == 0
    {
        result.push_str("Plang");
    }
    if n % 7 == 0
    {
        result.push_str("Plong");
    }

    if n % 3 == 1 && n % 5 == 1 && n % 5 == 1
    {
        return n.to_string();
    }

    return result

}

fn main()
{
    println!("Dear user, please enter a number!");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("Failed to read line");
    let number:u32 = number.trim().parse().expect("Type in a positive whole number");
    raindrops(number);
    
}
