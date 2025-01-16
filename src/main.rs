fn main() {
    let value = 42;

    match value {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Not one, two, or three"),
    }

    let option = Some(5);

    match option {
        Some(x) if x > 0 => println!("Positive {}", x),
        Some(_) => println!("Non-positive"),
        None => println!("No value"),
    }

    let tuple = (1, 2);

    match tuple {
        (0, y) => println!("First is zero, second is {}", y),
        (x, 0) => println!("First is {}, second is zero", x),
        (x, y) => println!("First is {}, second is {}", x, y),
    }

    let character = 'c';

    match character {
        'a' | 'b' | 'c' => println!("Found a, b, or c"),
        _ => println!("Other character"),
    }

    let number = -1;

    match number {
        n if n < 0 => println!("Negative number"),
        n if n == 0 => println!("Zero"),
        n => println!("Positive number: {}", n),
    }

    let random_value = "hello";

    match random_value {
        "hello" => println!("Hello there!"),
        "world" => println!("World!"),
        _ => println!("Unknown greeting"),
    }

    let list = vec![1, 2, 3];

    match list.as_slice() {
        [1, 2, ..] => println!("Starts with 1, 2"),
        [1, ..] => println!("Starts with 1"),
        [] => println!("Empty list"),
        _ => println!("Some other list"),
    }
}