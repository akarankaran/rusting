fn main() {
    let input = String::from("This is a sample string for splitting into words.");
    let words: Vec<&str> = input.split_whitespace().collect();
    
    for word in &words {
        println!("{}", word);
    }
}