fn get_element<T>(arr: &[T], index: usize) -> Option<&T> {
    if index < arr.len() {
        Some(&arr[index])
    } else {
        None
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    
    match get_element(&arr, 2) {
        Some(value) => println!("Element at index 2: {}", value),
        None => println!("Index out of bounds"),
    }
    
    match get_element(&arr, 5) {
        Some(value) => println!("Element at index 5: {}", value),
        None => println!("Index out of bounds"),
    }

    let str_arr = ["apple", "banana", "cherry"];
    
    match get_element(&str_arr, 1) {
        Some(value) => println!("Element at index 1: {}", value),
        None => println!("Index out of bounds"),
    }
    
    match get_element(&str_arr, 3) {
        Some(value) => println!("Element at index 3: {}", value),
        None => println!("Index out of bounds"),
    }

    let empty_arr: [i32; 0] = [];
    
    match get_element(&empty_arr, 0) {
        Some(value) => println!("Element at index 0: {}", value),
        None => println!("Index out of bounds"),
    }
}