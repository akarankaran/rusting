use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 10);
    scores.insert("Bob", 20);
    scores.insert("Carol", 30);

    let names = vec!["Alice", "Bob", "Carol", "Dave"];
    for name in names {
        match scores.get(name) {
            Some(&score) => println!("{} scored {}", name, score),
            None => println!("{} has no score", name),
        }
    }

    let number = 30;
    match number {
        1 | 2 | 3 => println!("Small number"),
        4..=10 => println!("Medium number"),
        11..=100 => println!("Large number"),
        _ => println!("Number out of range"),
    }

    let value = Some(7);
    match value {
        Some(0) => println!("Zero"),
        Some(n) if n < 10 => println!("Single digit: {}", n),
        Some(n) => println!("Double digit: {}", n),
        None => println!("No value"),
    }

    let option = None;
    match option {
        Some(val) => println!("Value is: {}", val),
        None => println!("No value present"),
    }

    let position = (1, 2);
    match position {
        (0, 0) => println!("At origin"),
        (0, y) => println!("On Y axis at {}", y),
        (x, 0) => println!("On X axis at {}", x),
        (x, y) => println!("At position ({}, {})", x, y),
    }
}