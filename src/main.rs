fn largest<T: PartialOrd>(vec: &[T]) -> Option<&T> {
    let mut largest: Option<&T> = None;

    for item in vec {
        largest = match largest {
            Some(l) if *item <= *l => largest,
            _ => Some(item),
        }
    }
    largest
}

fn main() {
    let numbers = vec![3, 5, 1, 8, 2];
    let largest_number = largest(&numbers);
    println!("The largest number is: {:?}", largest_number);

    let floats = vec![3.5, 2.7, 8.1, 2.9];
    let largest_float = largest(&floats);
    println!("The largest float is: {:?}", largest_float);

    let chars = vec!['a', 'b', 'z', 'k'];
    let largest_char = largest(&chars);
    println!("The largest character is: {:?}", largest_char);

    let empty: Vec<i32> = vec![];
    let largest_empty = largest(&empty);
    println!("The largest in an empty vector is: {:?}", largest_empty);
}