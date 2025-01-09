use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter first number:");
    io::stdin().read_line(&mut input).unwrap();
    let num1: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter second number:");
    io::stdin().read_line(&mut input).unwrap();
    let num2: f64 = input.trim().parse().unwrap();

    println!("Select operation: +, -, *, /");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let operation = input.trim();

    match operation {
        "+" => println!("Result: {}", num1 + num2),
        "-" => println!("Result: {}", num1 - num2),
        "*" => println!("Result: {}", num1 * num2),
        "/" => {
            if num2 != 0.0 {
                println!("Result: {}", num1 / num2);
            } else {
                println!("Error: Division by zero");
            }
        },
        _ => println!("Invalid operation"),
    }
}