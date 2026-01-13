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

    if n % 3 == 0 && n % 5 == 0 && n % 7 == 0
    {
        let result = "PlingPlangPlong".to_string();
        return result;
    }
    else if n % 3 == 0 && n % 5 == 0
    {
        let result = "PlingPlang".to_string();
        return result;
    }
    else if n % 3 == 0 && n % 7 == 0
    {
        let  result = "PlingPlong".to_string();
        return result;
    }
    else if n % 5 == 0 && n % 7 == 0
    {
        let result = "PlangPlong".to_string();
        return result;
    }
    else
    {
        return "null".to_string();
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
