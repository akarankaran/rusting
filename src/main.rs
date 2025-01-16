use std::vec::Vec;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let target = 3;

    if let Some(pos) = numbers.iter().position(|&x| x == target) {
        println!("Found {} at position {}", target, pos);
    } else {
        println!("{} not found in the vector", target);
    }

    let names = vec!["Alice", "Bob", "Charlie", "Diana"];
    let target_name = "Charlie";

    if names.contains(&target_name) {
        println!("{} is in the list", target_name);
    } else {
        println!("{} is not in the list", target_name);
    }

    let mixed = vec![10, "Hello", 20.5, "World"];
    let target_value = "World";

    if let Some(pos) = mixed.iter().position(|&x| x == target_value) {
        println!("Found '{}' at position {}", target_value, pos);
    } else {
        println!("'{}' not found in the vector", target_value);
    }

    let filtered: Vec<_> = numbers.iter().filter(|&&x| x > 3).collect();
    println!("Filtered numbers: {:?}", filtered);
}