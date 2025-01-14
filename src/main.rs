fn main() {
    let value1: Option<i32> = Some(10);
    let value2: Option<i32> = None;

    let result1 = value1.unwrap_or(0);
    let result2 = value2.unwrap_or(5);

    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);

    let value3: Option<&str> = Some("Hello");
    let value4: Option<&str> = None;

    let greeting1 = value3.unwrap_or("Default Greeting");
    let greeting2 = value4.unwrap_or("Hello, World!");

    println!("Greeting 1: {}", greeting1);
    println!("Greeting 2: {}", greeting2);

    let value5: Option<f64> = Some(3.14);
    let value6: Option<f64> = None;

    let pi1 = value5.unwrap_or(3.0);
    let pi2 = value6.unwrap_or(2.71);

    println!("Pi 1: {}", pi1);
    println!("Pi 2: {}", pi2);

    let value7: Option<String> = Some("Rust".to_string());
    let value8: Option<String> = None;

    let lang1 = value7.unwrap_or("Unknown Language".to_string());
    let lang2 = value8.unwrap_or("Rust is Great".to_string());

    println!("Language 1: {}", lang1);
    println!("Language 2: {}", lang2);
}