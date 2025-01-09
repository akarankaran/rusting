use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter three numbers separated by spaces:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter a valid number"))
        .collect();

    if numbers.len() != 3 {
        println!("Please enter exactly three numbers.");
        return;
    }

    let sum: f64 = numbers.iter().sum();
    let average = sum / 3.0;
    println!("The average is: {}", average);
}