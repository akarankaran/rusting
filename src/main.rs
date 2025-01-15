use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<f64> = input.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let average = calculate_average(&numbers);
    println!("Average: {}", average);
}

fn calculate_average(numbers: &[f64]) -> f64 {
    let sum: f64 = numbers.iter().sum();
    sum / numbers.len() as f64
}