fn main() {
    let number = 15;

    if number % 3 == 0 && number % 5 == 0 {
        println!("{} is divisible by both 3 and 5", number);
    } else {
        println!("{} is not divisible by both 3 and 5", number);
    }
}