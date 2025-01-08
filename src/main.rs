fn main() {
    let number: i32 = 5;

    if number > 0 {
        println!("Positive");
    } else if number < 0 {
        println!("Negative");
    } else {
        println!("Zero");
    }

    let numbers = [-10, 0, 15, -3, 2];

    for &num in &numbers {
        if num > 0 {
            println!("{} is Positive", num);
        } else if num < 0 {
            println!("{} is Negative", num);
        } else {
            println!("{} is Zero", num);
        }
    }

    let num = -1;

    match num {
        n if n > 0 => println!("Positive"),
        n if n < 0 => println!("Negative"),
        _ => println!("Zero"),
    }

    let inputs = vec![3, -1, 0, 45, -22];

    for &input in &inputs {
        if input > 0 {
            println!("{} is Positive", input);
        } else if input < 0 {
            println!("{} is Negative", input);
        } else {
            println!("{} is Zero", input);
        }
    }

    let value = 0;

    match value {
        n if n > 0 => println!("Value is Positive"),
        n if n < 0 => println!("Value is Negative"),
        _ => println!("Value is Zero"),
    }
}