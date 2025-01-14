use std::num::ParseIntError;
use std::io;

fn main() {
    match do_something() {
        Ok(_) => println!("Operation succeeded."),
        Err(e) => handle_error(e),
    }
}

fn do_something() -> Result<(), Box<dyn std::error::Error>> {
    let input = "42a";
    let _num: i32 = input.parse()?;
    Ok(())
}

fn handle_error(e: Box<dyn std::error::Error>) {
    match e.downcast_ref::<ParseIntError>() {
        Some(_) => println!("Failed to parse integer: {}", e),
        None => match e.downcast_ref::<io::Error>() {
            Some(_) => println!("I/O error occurred: {}", e),
            None => println!("An unexpected error occurred: {}", e),
        },
    }
}