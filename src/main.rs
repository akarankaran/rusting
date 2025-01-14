use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();
        print!("Please enter a number: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(num) => {
                println!("You entered the number: {}", num);
                break;
            }
            Err(_) => {
                eprintln!("Error: Invalid input, please enter a valid integer.");
            }
        }
    }

    loop {
        let mut choice = String::new();
        print!("Do you want to continue? (y/n): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim().to_lowercase().as_str() {
            "y" => {
                println!("Continuing...");
                break;
            }
            "n" => {
                println!("Exiting program.");
                return;
            }
            _ => {
                eprintln!("Error: Invalid choice. Please enter 'y' or 'n'.");
            }
        }
    }

    loop {
        let mut age_input = String::new();
        print!("Please enter your age: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut age_input).expect("Failed to read line");

        match age_input.trim().parse::<u32>() {
            Ok(age) => {
                if age < 0 {
                    eprintln!("Error: Age cannot be negative.");
                } else {
                    println!("Your age is: {}", age);
                    break;
                }
            }
            Err(_) => {
                eprintln!("Error: Please enter a valid positive number for age.");
            }
        }
    }
}