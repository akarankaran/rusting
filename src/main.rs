use std::io::{self, Write};

enum Command {
    Exit,
    Greet(String),
    Add(i32, i32),
    Subtract(i32, i32),
    Help,
}

fn parse_command(input: &str) -> Option<Command> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    match parts.as_slice() {
        ["exit"] => Some(Command::Exit),
        ["greet", name] => Some(Command::Greet(name.to_string())),
        ["add", a, b] => {
            if let (Ok(x), Ok(y)) = (a.parse(), b.parse()) {
                Some(Command::Add(x, y))
            } else {
                None
            }
        }
        ["subtract", a, b] => {
            if let (Ok(x), Ok(y)) = (a.parse(), b.parse()) {
                Some(Command::Subtract(x, y))
            } else {
                None
            }
        }
        ["help"] => Some(Command::Help),
        _ => None,
    }
}

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match parse_command(&input) {
            Some(Command::Exit) => {
                println!("Exiting...");
                break;
            }
            Some(Command::Greet(name)) => {
                println!("Hello, {}!", name);
            }
            Some(Command::Add(a, b)) => {
                println!("Result: {}", a + b);
            }
            Some(Command::Subtract(a, b)) => {
                println!("Result: {}", a - b);
            }
            Some(Command::Help) => {
                println!("Available commands: exit, greet <name>, add <a> <b>, subtract <a> <b>, help");
            }
            None => {
                println!("Unknown command. Type 'help' for available commands.");
            }
        }
    }
}