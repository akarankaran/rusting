use std::fs;
use std::num::ParseIntError;

enum CustomError {
    IoError(std::io::Error),
    ParseError(ParseIntError),
    OtherError(String),
}

fn read_file(file_path: &str) -> Result<String, CustomError> {
    fs::read_to_string(file_path).map_err(CustomError::IoError)
}

fn parse_number(num_str: &str) -> Result<i32, CustomError> {
    num_str.parse::<i32>().map_err(CustomError::ParseError)
}

fn perform_operations(file_path: &str) -> Result<i32, CustomError> {
    let content = read_file(file_path)?;
    let number = parse_number(&content.trim())?;
    Ok(number * 2)
}

fn main() {
    let file_path = "numbers.txt";
    match perform_operations(file_path) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => match e {
            CustomError::IoError(err) => eprintln!("IO Error: {}", err),
            CustomError::ParseError(err) => eprintln!("Parse Error: {}", err),
            CustomError::OtherError(msg) => eprintln!("Error: {}", msg),
        },
    }
}