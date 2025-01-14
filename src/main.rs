use std::num::ParseIntError;

fn convert_to_number(input: &str) -> Result<i32, ParseIntError> {
    input.trim().parse::<i32>()
}

fn main() {
    let inputs = vec!["42", "  -42  ", "abc", "3.14", "0", "", "2147483647", "-2147483648", "2147483648"];

    for &input in &inputs {
        match convert_to_number(input) {
            Ok(number) => println!("Successfully converted '{}': {}", input, number),
            Err(e) => println!("Failed to convert '{}': {}", input, e),
        }
    }
}