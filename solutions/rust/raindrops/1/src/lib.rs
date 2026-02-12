pub fn raindrops(n: u32) -> String {

    if n % 3 == 0
    {
        let result = "Pling".to_string();
        return result;
    }
    else if n % 5 == 0
    {
        let result = "Plang".to_string();
        return result;
    }
    else if n % 7 == 0 
    {
        let result = "Plong".to_string();
        return result;
    }
    else
    {
        let result = n.to_string();
        return result;
    }
}

fn main()
{
    println!("Dear user, please enter a number!");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("Failed to read line");
    let number:u32 = number.trim().parse().expect("Type in a positive whole number");
    raindrops(number);
}
