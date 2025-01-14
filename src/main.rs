use std::fmt;

#[derive(Debug)]
enum MyError {
    NotFound,
    InvalidInput(String),
    InternalError,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::NotFound => write!(f, "Resource not found"),
            MyError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            MyError::InternalError => write!(f, "An internal error occurred"),
        }
    }
}

impl std::error::Error for MyError {}

fn find_item(id: usize) -> Result<&'static str, MyError> {
    match id {
        1 => Ok("Item 1"),
        2 => Ok("Item 2"),
        _ => Err(MyError::NotFound),
    }
}

fn parse_number(input: &str) -> Result<i32, MyError> {
    input.parse::<i32>().map_err(|_| MyError::InvalidInput(input.to_string()))
}

fn perform_operation(num: i32) -> Result<i32, MyError> {
    if num < 0 {
        Err(MyError::InternalError)
    } else {
        Ok(num * 2)
    }
}

fn main() {
    let item_result = find_item(3);
    match item_result {
        Ok(item) => println!("Found: {}", item),
        Err(e) => println!("Error: {}", e),
    }

    let parse_result = parse_number("42a");
    match parse_result {
        Ok(number) => println!("Parsed number: {}", number),
        Err(e) => println!("Error: {}", e),
    }

    let operation_result = perform_operation(-5);
    match operation_result {
        Ok(result) => println!("Operation result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}