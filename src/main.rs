fn main() {
    let input_string = String::from("This is a sample string with spaces.");
    let space_count = input_string.chars().filter(|&c| c == ' ').count();
    println!("Number of spaces: {}", space_count);
}