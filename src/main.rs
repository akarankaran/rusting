use std::collections::HashMap;

fn main() {
    let words = vec!["apple", "banana", "cherry", "date", "elderberry"];
    let target = "cherry";

    for &word in &words {
        match word {
            w if w == target => println!("Found: {}", w),
            _ => {}
        }
    }

    let mut word_map: HashMap<String, usize> = HashMap::new();
    for word in words {
        *word_map.entry(word.to_string()).or_insert(0) += 1;
    }

    match word_map.get(target) {
        Some(&count) => println!("'{}' appears {} time(s)", target, count),
        None => println!("'{}' not found", target),
    }
}