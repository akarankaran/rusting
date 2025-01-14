use regex::Regex;
use std::io;

fn main() {
    let email = get_input("Enter your email: ");
    match validate_email(&email) {
        Ok(_) => println!("Valid email format."),
        Err(e) => println!("Error: {}", e),
    }

    let phone = get_input("Enter your phone number: ");
    match validate_phone(&phone) {
        Ok(_) => println!("Valid phone format."),
        Err(e) => println!("Error: {}", e),
    }
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn validate_email(email: &str) -> Result<(), &str> {
    let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if re.is_match(email) {
        Ok(())
    } else {
        Err("Invalid email format.")
    }
}

fn validate_phone(phone: &str) -> Result<(), &str> {
    let re = Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();
    if re.is_match(phone) {
        Ok(())
    } else {
        Err("Invalid phone number format.")
    }
}