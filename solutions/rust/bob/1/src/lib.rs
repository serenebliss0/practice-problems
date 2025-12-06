use std::io;

pub fn reply(message: &str) -> String
 {

    let mut reply = String::new();
    
    let message_uppercase = message.chars()
            .filter(|c| c.is_alphabetic())
            .all(|c| c.is_uppercase());

    if message.contains("?")
    {
        reply.push_str("Sure");
    }
    else if message_uppercase == true
    {
        reply.push_str("Whoa, chill out!");
    }
    else if message.contains("?") && message_uppercase == true
    {
        reply.push_str("Calm down, I know what I'm doing!");
    }
    else if message.is_empty()
    {
        reply.push_str("Fine. Be that way");
    }
    else
    {

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

    reply(message);

}