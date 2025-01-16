fn main() {
    let some_value = Some(10);
    if let Some(val) = some_value {
        println!("Value is: {}", val);
    }

    let none_value: Option<i32> = None;
    if let Some(val) = none_value {
        println!("This won't print: {}", val);
    } else {
        println!("No value found.");
    }

    let some_string = Some("Hello");
    if let Some(text) = some_string {
        println!("{}", text);
    }

    let none_string: Option<&str> = None;
    if let Some(text) = none_string {
        println!("{}", text);
    } else {
        println!("No text found.");
    }

    let some_float: Option<f64> = Some(3.14);
    if let Some(num) = some_float {
        println!("Float value: {}", num);
    }

    let arr: [Option<i32>; 5] = [Some(1), None, Some(2), None, Some(3)];
    for item in arr.iter() {
        if let Some(value) = item {
            println!("Array value: {}", value);
        }
    }

    let map = std::collections::HashMap::new();
    let some_key = "key";
    if let Some(value) = map.get(some_key) {
        println!("Found in map: {}", value);
    } else {
        println!("Key not found in map.");
    }
}