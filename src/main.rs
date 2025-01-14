fn parse_integer(input: &str) -> Result<i32, &'static str> {
    match input.trim().parse::<i32>() {
        Ok(value) => Ok(value),
        Err(_) => Err("Failed to parse input into an integer"),
    }
}

fn main() {
    let inputs = vec!["42", "   -42", "abc", "100.5", "   ", ""];

    for input in inputs {
        match parse_integer(input) {
            Ok(value) => println!("Parsed value: {}", value),
            Err(e) => println!("Error: {}", e),
        }
    }
}