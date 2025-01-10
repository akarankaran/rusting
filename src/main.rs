fn first_character(s: &str) -> Option<char> {
    s.chars().next()
}

fn main() {
    let example_string = "Hello, world!";
    match first_character(example_string) {
        Some(c) => println!("The first character is: {}", c),
        None => println!("The string is empty."),
    }
}