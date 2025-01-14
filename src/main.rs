use std::option::Option;

fn sqrt(num: f64) -> Option<f64> {
    if num < 0.0 {
        None
    } else {
        Some(num.sqrt())
    }
}

fn main() {
    let numbers = vec![4.0, 9.0, -1.0, 16.0, 25.0, -36.0];

    for &number in &numbers {
        match sqrt(number) {
            Some(result) => println!("The square root of {} is {}", number, result),
            None => println!("Cannot calculate the square root of negative number: {}", number),
        }
    }
}