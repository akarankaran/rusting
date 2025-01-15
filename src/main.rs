use std::collections::HashMap;

fn main() {
    let array = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let mut frequency = HashMap::new();

    for &item in &array {
        *frequency.entry(item).or_insert(0) += 1;
    }

    for (key, count) in frequency {
        println!("{}: {}", key, count);
    }
    
    let array_of_strings = vec!["apple", "banana", "apple", "orange", "banana", "banana"];
    let mut string_frequency = HashMap::new();

    for &item in &array_of_strings {
        *string_frequency.entry(item).or_insert(0) += 1;
    }

    for (key, count) in string_frequency {
        println!("{}: {}", key, count);
    }

    let mixed_array = vec![1, "apple", 2, 1, "banana", "apple"];
    let mut mixed_frequency = HashMap::new();

    for item in mixed_array {
        *mixed_frequency.entry(item).or_insert(0) += 1;
    }

    for (key, count) in mixed_frequency {
        println!("{}: {}", key, count);
    }

    let empty_array: Vec<i32> = vec![];
    let mut empty_frequency = HashMap::new();

    for &item in &empty_array {
        *empty_frequency.entry(item).or_insert(0) += 1;
    }

    if empty_frequency.is_empty() {
        println!("Array is empty. No frequencies to display.");
    }
}