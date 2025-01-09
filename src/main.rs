fn main() {
    let input_string = String::from("Hello, world!");
    let vowels = "aeiouAEIOU";
    let mut count = 0;

    for c in input_string.chars() {
        if vowels.contains(c) {
            count += 1;
        }
    }

    println!("Number of vowels: {}", count);
}