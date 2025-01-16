fn main() {
    let value = true;

    match value {
        true => println!("Yes"),
        false => println!("No"),
    }

    let another_value = false;

    if another_value {
        println!("Yes");
    } else {
        println!("No");
    }

    let conditional_value = 5 > 3;

    println!("{}", if conditional_value { "Yes" } else { "No" });

    let is_ready = true;

    match is_ready {
        true => println!("Yes"),
        _ => println!("No"),
    }

    let check_value: bool = std::env::args().nth(1).unwrap_or_else(|| "false".to_string()).parse().unwrap();

    if check_value {
        println!("Yes");
    } else {
        println!("No");
    }
}