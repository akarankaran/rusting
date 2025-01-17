use std::collections::HashMap;

fn main() {
    let text = "Hello world Hello Rust Rust is amazing Rust is fast";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    for (word, count) in &word_count {
        println!("{}: {}", word, count);
    }
}