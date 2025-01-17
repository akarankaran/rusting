use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn parse_integer(input: &str) -> Result<i32, ParseIntError> {
    input.parse::<i32>()
}

fn main() {
    match read_file_content("example.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    let inputs = vec!["42", "not a number", "100"];

    for input in inputs {
        match parse_integer(input) {
            Ok(num) => println!("Parsed number: {}", num),
            Err(e) => eprintln!("Failed to parse '{}': {}", input, e),
        }
    }

    let division = |num: i32| -> Result<i32, &'static str> {
        if num == 0 {
            Err("Attempted to divide by zero")
        } else {
            Ok(42 / num)
        }
    };

    let numbers = vec![2, 0, 7];

    for number in numbers {
        match division(number) {
            Ok(result) => println!("Division result: {}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}