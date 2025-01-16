fn main() {
    let numbers = vec![5, -3, 0, 42, -1, 0, 7, -100, 15];

    for number in numbers {
        match number {
            n if n > 0 => println!("{} is positive", n),
            n if n < 0 => println!("{} is negative", n),
            _ => println!("{} is zero", number),
        }
    }
}