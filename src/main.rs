fn main() {
    let value: Option<i32> = Some(10);
    
    let result = value
        .and_then(|x| if x > 5 { Some(x * 2) } else { None })
        .and_then(|x| Some(x + 3))
        .and_then(|x| if x < 30 { Some(x / 2) } else { None })
        .and_then(|x| Some(x - 1));
    
    match result {
        Some(res) => println!("Result: {}", res),
        None => println!("No valid result"),
    }

    let none_value: Option<i32> = None;
    
    let none_result = none_value
        .and_then(|x| Some(x + 5))
        .and_then(|x| Some(x * 2));
    
    match none_result {
        Some(res) => println!("Result: {}", res),
        None => println!("No valid result"),
    }
    
    let another_value: Option<i32> = Some(6);
    
    let another_result = another_value
        .and_then(|x| if x % 2 == 0 { Some(x / 2) } else { None })
        .and_then(|x| Some(x + 1))
        .and_then(|x| Some(x * 3));
    
    match another_result {
        Some(res) => println!("Result: {}", res),
        None => println!("No valid result"),
    }
}