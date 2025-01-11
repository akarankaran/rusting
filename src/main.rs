fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for number in numbers {
        if is_even(number) {
            println!("{} is even", number);
        } else {
            println!("{} is odd", number);
        }
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}