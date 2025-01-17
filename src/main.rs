use std::fs::File;
use std::io::{self, Read};

enum CustomError {
    IoError(io::Error),
    OtherError(String),
}

fn read_file_content(file_path: &str) -> Result<String, CustomError> {
    let mut file = File::open(file_path).map_err(CustomError::IoError)?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(CustomError::IoError)?;
    Ok(content)
}

fn main() {
    match read_file_content("example.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(CustomError::IoError(e)) => println!("I/O error occurred: {}", e),
        Err(CustomError::OtherError(msg)) => println!("An error occurred: {}", msg),
    }

    let result: Result<i32, &str> = divide(10, 0);
    match result {
        Ok(value) => println!("Result of division: {}", value),
        Err(e) => println!("Error occurred: {}", e),
    }
}

fn divide(numerator: i32, denominator: i32) -> Result<i32, &'static str> {
    if denominator == 0 {
        Err("Cannot divide by zero")
    } else {
        Ok(numerator / denominator)
    }
}