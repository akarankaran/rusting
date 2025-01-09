use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a number:");
    io::stdin().read_line(&mut input).unwrap();
    let mut number: i32 = input.trim().parse().unwrap();
    let mut sum = 0;

    while number > 0 {
        sum += number % 10;
        number /= 10;
    }

    println!("The sum of the digits is: {}", sum);
}