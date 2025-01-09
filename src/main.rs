use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let total_seconds: u64 = input.trim().parse().expect("Please enter a valid number");

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    println!("{} seconds is {} hours, {} minutes, and {} seconds.", total_seconds, hours, minutes, seconds);
}