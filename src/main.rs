fn main() {
    let numbers = [34, 15, 88, 2, -5, 0];
    let mut smallest = numbers[0];

    for &number in &numbers {
        if number < smallest {
            smallest = number;
        }
    }

    println!("The smallest number is {}", smallest);
}