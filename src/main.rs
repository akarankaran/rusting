fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    let index = 2;
    let value = vec.get(index).unwrap_or(&0);
    println!("Value at index {}: {}", index, value);

    let out_of_bounds_index = 10;
    match vec.get(out_of_bounds_index) {
        Some(&val) => println!("Value at index {}: {}", out_of_bounds_index, val),
        None => println!("No value at index {}", out_of_bounds_index),
    }

    let first_element = vec.get(0).copied().unwrap_or(-1);
    println!("First element: {}", first_element);

    let second_element = vec.get(1).map(|&v| v * 2).unwrap_or(0);
    println!("Twice the second element: {}", second_element);

    let last_element = vec.get(vec.len().checked_sub(1).unwrap_or(0)).unwrap_or(&-1);
    println!("Last element: {}", last_element);

    let index_to_access = 4;
    let access_result: Option<&i32> = vec.get(index_to_access);
    match access_result {
        Some(&val) if val % 2 == 0 => println!("Element at index {} is even: {}", index_to_access, val),
        Some(&val) => println!("Element at index {} is odd: {}", index_to_access, val),
        None => println!("No element found at index {}", index_to_access),
    }

    let elements: Vec<Option<i32>> = (0..10).map(|i| vec.get(i).copied()).collect();
    for (i, elem) in elements.iter().enumerate() {
        match elem {
            Some(&val) => println!("Element at index {}: {}", i, val),
            None => println!("No element at index {}", i),
        }
    }
}