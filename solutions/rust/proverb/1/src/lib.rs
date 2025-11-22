pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    for i in 0..list.len().saturating_sub(1) 
    {
        let line = format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]);
        proverb.push_str(&line);
    }

    if !list.is_empty() 
    {
        let last_line = format!("And all for the want of a {}.", list[0]);
        proverb.push_str(&last_line);
    }

    proverb
}



fn main()
{
    let mut list = Vec::new();
    println!("How many inputs do you want to have?");
    let mut number_of_inputs = String::new();
    std::io::stdin().read_line(&mut number_of_inputs).expect("Failed to read line");
    let number_of_inputs:u8 = number_of_inputs.trim().parse().expect("Enter a positive integer");

    let x = 1;
    for x in x..=number_of_inputs
    {
        println!("Enter input {}", x);
        let mut input_1 = String::new();
        std::io::stdin().read_line(&mut input_1).expect("Failed to read line");
        let mut message = input_1;
        list.push(message.trim().to_string());
    }
}
