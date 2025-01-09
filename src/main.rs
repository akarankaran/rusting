fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let mut product = 1;

    for &num in &numbers {
        product *= num;
    }

    println!("The product of all numbers in the array is: {}", product);
}