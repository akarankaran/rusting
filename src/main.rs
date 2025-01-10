fn main() {
    let s = String::from("Hello, world!");
    let length = calculate_length(&s);
    println!("The length of '{}' is {}.", s, length);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}