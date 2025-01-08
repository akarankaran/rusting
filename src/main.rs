use std::f64::consts::PI;
use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter the radius of the circle:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let radius: f64 = input.trim().parse().expect("Please enter a valid number");
    let area = calculate_area(radius);
    println!("The area of the circle with radius {} is {}", radius, area);
}

fn calculate_area(radius: f64) -> f64 {
    PI * radius * radius
}