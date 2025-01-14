use std::fs::File;
use std::io::{self, Read};

fn main() {
    let file_path = "example.txt";
    let mut file = File::open(file_path).unwrap();
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    println!("File contents:\n{}", contents);

    let number: i32 = "42".parse().unwrap();
    println!("Parsed number: {}", number);

    let vec = vec![1, 2, 3];
    let second_element = vec.get(1).unwrap();
    println!("Second element: {}", second_element);

    let json_str = r#"{"name": "Alice", "age": 30}"#;
    let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap();
    println!("Parsed JSON: {:?}", parsed);
}