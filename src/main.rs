use std::io;

fn main() {
    let number = read_number();
    match number {
        Ok(n) => println!("You entered: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_number() -> Result<i32, String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|_| "Failed to read line".to_string())?;
    let trimmed = input.trim();
    trimmed.parse::<i32>().map_err(|_| "Invalid input, please enter a number".to_string())
}