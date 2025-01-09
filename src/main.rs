fn main() {
    let mut number = 12345;
    let mut reversed = 0;

    while number != 0 {
        let digit = number % 10;
        reversed = reversed * 10 + digit;
        number /= 10;
    }

    println!("{}", reversed);
}