fn main() {
    let string1 = String::from("Hello, ");
    let string2 = String::from("world!");
    let concatenated = string1 + &string2;
    println!("{}", concatenated);
}