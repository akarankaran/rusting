use std::vec::Vec;

fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    for i in 1..=10 {
        numbers.push(i);
    }

    for number in &numbers {
        println!("{}", number);
    }
}