use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    
    let key_to_remove = "b";
    map.remove(key_to_remove);
    
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    let key_to_remove_2 = "d";
    map.remove(key_to_remove_2);

    let new_key = "e";
    map.insert(new_key, 5);

    map.retain(|&k, _| k != "a");

    if let Some(value) = map.get("c") {
        println!("Found value: {}", value);
    }

    map.clear();
}