fn split_array<T>(array: &[T]) -> (Vec<T>, Vec<T>) 
where 
    T: Clone,
{
    let mid = array.len() / 2;
    let first_half = array[..mid].to_vec();
    let second_half = array[mid..].to_vec();
    (first_half, second_half)
}

fn main() {
    let array = vec![1, 2, 3, 4, 5, 6];
    let (first_half, second_half) = split_array(&array);
    println!("{:?}, {:?}", first_half, second_half);

    let string_array = vec!["a", "b", "c", "d", "e"];
    let (first_half_strings, second_half_strings) = split_array(&string_array);
    println!("{:?}, {:?}", first_half_strings, second_half_strings);

    let mixed_array: Vec<Box<dyn std::any::Any>> = vec![Box::new(1), Box::new("hello"), Box::new(3.14)];
    let (first_half_mixed, second_half_mixed) = split_array(&mixed_array);
    println!("{:?}, {:?}", first_half_mixed.len(), second_half_mixed.len());
}