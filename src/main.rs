use std::collections::HashMap;

fn main() {
    let encoded_message = "72 101 108 108 111 32 87 111 114 108 100";
    let decoded_message = decode_message(encoded_message);
    println!("{}", decoded_message);
}

fn decode_message(encoded: &str) -> String {
    let mut decoded = String::new();
    for number in encoded.split_whitespace() {
        if let Ok(ascii) = number.parse::<u8>() {
            decoded.push(ascii as char);
        }
    }
    decoded
}

fn example_patterns() {
    let patterns: HashMap<&str, &str> = [
        ("72", "H"),
        ("101", "e"),
        ("108", "l"),
        ("111", "o"),
        ("32", " "),
        ("87", "W"),
        ("111", "o"),
        ("114", "r"),
        ("108", "l"),
        ("100", "d"),
    ].iter().cloned().collect();
    
    for (key, value) in &patterns {
        println!("ASCII: {}, Character: {}", key, value);
    }
}

fn advanced_decoding(encoded: &str) -> String {
    encoded.split_whitespace()
           .filter_map(|s| s.parse::<u8>().ok())
           .map(|ascii| ascii as char)
           .collect()
}