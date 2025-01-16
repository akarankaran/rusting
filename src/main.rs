use std::io;

fn main() {
    println!("Enter the temperature in Celsius:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let temperature: f32 = input.trim().parse().expect("Please enter a valid number");

    match temperature {
        t if t <= 0.0 => println!("Cold"),
        t if t > 0.0 && t <= 15.0 => println!("Cool"),
        t if t > 15.0 && t <= 25.0 => println!("Warm"),
        t if t > 25.0 && t <= 35.0 => println!("Hot"),
        _ => println!("Very Hot"),
    }
}