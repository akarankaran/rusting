use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a number to convert to binary:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number: u32 = input.trim().parse().expect("Please enter a valid number");
    let mut binary = Vec::new();
    
    let mut num = number;
    if num == 0 {
        binary.push('0');
    } else {
        while num > 0 {
            binary.push(if num % 2 == 0 { '0' } else { '1' });
            num /= 2;
        }
    }
    
    binary.reverse();
    let binary_string: String = binary.iter().collect();
    println!("Binary representation of {} is: {}", number, binary_string);
}