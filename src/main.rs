use std::collections::HashSet;

fn main() {
    let input_vector = vec![1, 2, 2, 3, 4, 4, 5];
    let unique_elements: HashSet<_> = input_vector.into_iter().collect();
    let unique_vector: Vec<_> = unique_elements.into_iter().collect();
    println!("{:?}", unique_vector);

    let input_strings = vec!["apple", "banana", "apple", "orange"];
    let unique_strings: HashSet<_> = input_strings.into_iter().collect();
    let unique_strings_vector: Vec<_> = unique_strings.into_iter().collect();
    println!("{:?}", unique_strings_vector);

    let input_floats = vec![1.1, 2.2, 2.2, 3.3, 4.4];
    let unique_floats: HashSet<_> = input_floats.into_iter().collect();
    let unique_floats_vector: Vec<_> = unique_floats.into_iter().collect();
    println!("{:?}", unique_floats_vector);
    
    let input_mixed = vec!["rust", 1, 2.2, "rust", "go"];
    let unique_mixed: HashSet<_> = input_mixed.into_iter().collect();
    let unique_mixed_vector: Vec<_> = unique_mixed.into_iter().collect();
    println!("{:?}", unique_mixed_vector);
}