use std::io;

pub fn reply(message: &str) -> String {
    let message = message.trim(); // remove whitespace

    if message.is_empty() {
        return "Fine. Be that way!".to_string();
    }

    let is_question = message.ends_with('?');
    let is_yelling = message.chars()
        .any(|c| c.is_alphabetic()) && message.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());

    if is_question && is_yelling {
        "Calm down, I know what I'm doing!".to_string()
    } else if is_question {
        "Sure.".to_string()
    } else if is_yelling {
        "Whoa, chill out!".to_string()
    } else {
        "Whatever.".to_string()
    }
}

pub fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim_end().to_string()
}

fn main() {
    println!("You're having a conversation with Bob.\nWhat would you like to say to him?");
    let message = read_line();
    let response = reply(&message);
    println!("Bob says: {}", response);
}
