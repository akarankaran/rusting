fn largest<T: PartialOrd>(vec: &[T]) -> &T {
    let mut largest = &vec[0];
    for item in vec {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number is {}", result);
}