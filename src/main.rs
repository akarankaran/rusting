use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a character:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let ch = input.trim().chars().next().unwrap_or(' ');

    if ch.is_alphabetic() && ch.len_utf8() == 1 {
        match ch.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("{} is a vowel.", ch),
            _ => println!("{} is a consonant.", ch),
        }
    } else {
        println!("Input is not a valid alphabetic character.");
    }
}