use std::io;

enum Command {
    Greet(String),
    Add(i32, i32),
    Subtract(i32, i32),
    Exit,
}

fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    match parts.as_slice() {
        ["greet", name] => Command::Greet(name.to_string()),
        ["add", a, b] => Command::Add(a.parse().unwrap(), b.parse().unwrap()),
        ["subtract", a, b] => Command::Subtract(a.parse().unwrap(), b.parse().unwrap()),
        ["exit"] => Command::Exit,
        _ => panic!("Unknown command"),
    }
}

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match parse_command(&input) {
            Command::Greet(name) => println!("Hello, {}!", name),
            Command::Add(a, b) => println!("Result: {}", a + b),
            Command::Subtract(a, b) => println!("Result: {}", a - b),
            Command::Exit => {
                println!("Goodbye!");
                break;
            }
        }
    }
}