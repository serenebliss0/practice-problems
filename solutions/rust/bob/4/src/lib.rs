use std::io;

pub fn reply(message: &str) -> String
 {

    let mut reply = String::new();

    pub fn reply(message: &str) -> String {
        let is_number = message.trim().parse::<i32>().is_ok();
    
        if is_number {
            return "Numbers? Not really my thing.".to_string();
        }
    
        let message_uppercase = message.chars()
            .filter(|c| c.is_alphabetic())
            .all(|c| c.is_uppercase());
    
        if message.trim().is_empty() {
            "Fine. Be that way!".to_string()
        } else if message.contains("?") && message_uppercase {
            "Calm down, I know what I'm doing!".to_string()
        } else if message.contains("?") {
            "Sure.".to_string()
        } else if message_uppercase {
            "Whoa, chill out!".to_string()
        } else {
            "Whatever.".to_string()
        }
    }
    reply
}

pub fn read_line() -> String
{
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Failed to read line");
    let option = option.trim();

    option.to_string()
}

fn main()
{
    println!("You're having a conversation with bob.\nWhat would you like to say to him?");
    println!("Be very specific with your message");
    let binding = read_line();
    let message = binding.as_str();

    let response = reply(message);
    println!("{}", response);

}