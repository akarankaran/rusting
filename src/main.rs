fn main() {
    let strings = vec!["", "a", "hello", "Rust", "programming", "example"];
    
    for s in strings {
        match s.len() {
            0 => println!("Empty string"),
            1 => println!("Single character: {}", s),
            2..=4 => println!("Short string: {}", s),
            5..=10 => println!("Medium string: {}", s),
            _ => println!("Long string: {}", s),
        }
    }
}