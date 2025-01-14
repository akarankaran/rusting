use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    let result = get_value(&map, "key3");
    match result {
        Some(value) => println!("Found: {}", value),
        None => println!("Key not found"),
    }
}

fn get_value<'a>(map: &'a HashMap<&str, &'a str>, key: &str) -> Option<&'a str> {
    map.get(key).cloned()
}