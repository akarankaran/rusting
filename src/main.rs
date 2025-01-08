[package]
name = "circle_area"
version = "0.1.0"
edition = "2021"

[dependencies]

[[bin]]
name = "circle_area"
path = "src/main.rs"

fn main() {
    let radius = 5.0;
    let area = calculate_area(radius);
    println!("The area of the circle is: {}", area);
}

fn calculate_area(radius: f64) -> f64 {
    std::f64::consts::PI * radius.powi(2)
}

fn calculate_area_with_input() {
    use std::io;
    let mut input = String::new();
    println!("Enter the radius of the circle:");
    io::stdin().read_line(&mut input).unwrap();
    let radius: f64 = input.trim().parse().unwrap();
    let area = calculate_area(radius);
    println!("The area of the circle is: {}", area);
}

fn calculate_area_multiple_radii(radii: Vec<f64>) {
    for radius in radii {
        let area = calculate_area(radius);
        println!("The area of the circle with radius {} is: {}", radius, area);
    }
}

fn calculate_area_from_file(file_path: &str) {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let radius: f64 = line.unwrap().trim().parse().unwrap();
        println!("The area of the circle with radius {} is: {}", radius, calculate_area(radius));
    }
}