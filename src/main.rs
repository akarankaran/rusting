fn main() {
    let s = String::from("Hello, world!");
    let length = print_length(&s);
    println!("The length of the string is: {}", length);
}

fn print_length(s: &str) -> usize {
    s.len()
}