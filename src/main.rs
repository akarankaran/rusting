fn main() {
    let some_value: Option<Option<i32>> = Some(Some(10));
    match some_value {
        Some(Some(x)) => println!("Value: {}", x),
        Some(None) => println!("Inner option is None."),
        None => println!("Outer option is None."),
    }

    let none_value: Option<Option<i32>> = Some(None);
    match none_value {
        Some(Some(x)) => println!("Value: {}", x),
        Some(None) => println!("Inner option is None."),
        None => println!("Outer option is None."),
    }

    let another_value: Option<Option<i32>> = None;
    match another_value {
        Some(Some(x)) => println!("Value: {}", x),
        Some(None) => println!("Inner option is None."),
        None => println!("Outer option is None."),
    }

    let empty_value: Option<Option<i32>> = Some(None);
    if let Some(Some(x)) = empty_value {
        println!("Value: {}", x);
    } else if let Some(None) = empty_value {
        println!("Inner option is None.");
    } else {
        println!("Outer option is None.");
    }

    let deep_value: Option<Option<Option<i32>>> = Some(Some(Some(20)));
    match deep_value {
        Some(Some(Some(x))) => println!("Deep Value: {}", x),
        Some(Some(None)) => println!("Second inner option is None."),
        Some(None) => println!("First inner option is None."),
        None => println!("Outer option is None."),
    }

    let complex_option: Option<Option<Option<i32>>> = None;
    match complex_option {
        Some(Some(Some(x))) => println!("Deep Value: {}", x),
        Some(Some(None)) => println!("Second inner option is None."),
        Some(None) => println!("First inner option is None."),
        None => println!("Outer option is None."),
    }

    let mixed_option: Option<Option<Option<i32>>> = Some(None);
    match mixed_option {
        Some(Some(Some(x))) => println!("Deep Value: {}", x),
        Some(Some(None)) => println!("Second inner option is None."),
        Some(None) => println!("First inner option is None."),
        None => println!("Outer option is None."),
    }
}