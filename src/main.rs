use std::io;

fn main() {
    let phone_number = get_input("Enter a phone number: ");
    match validate_phone_number(&phone_number) {
        Ok(_) => println!("Valid phone number."),
        Err(err) => println!("Invalid phone number: {}", err),
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    input.trim().to_string()
}

fn validate_phone_number(phone: &str) -> Result<(), String> {
    match phone.len() {
        10 => validate_length(phone),
        7 => validate_length(phone),
        _ => Err("Phone number must be 7 or 10 digits.".to_string()),
    }
}

fn validate_length(phone: &str) -> Result<(), String> {
    if phone.chars().all(char::is_numeric) {
        Ok(())
    } else {
        Err("Phone number must contain only digits.".to_string())
    }
}