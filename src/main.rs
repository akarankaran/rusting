use std::option::Option;

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let search_element = 3;
    let result = find_element(&arr, search_element);
    match result {
        Some(index) => println!("Element found at index: {}", index),
        None => println!("Element not found"),
    }
}

fn find_element(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &value) in arr.iter().enumerate() {
        if value == target {
            return Some(index);
        }
    }
    None
} 

fn find_string(arr: &[&str], target: &str) -> Option<usize> {
    for (index, &value) in arr.iter().enumerate() {
        if value == target {
            return Some(index);
        }
    }
    None
}

fn find_float(arr: &[f64], target: f64) -> Option<usize> {
    for (index, &value) in arr.iter().enumerate() {
        if (value - target).abs() < f64::EPSILON {
            return Some(index);
        }
    }
    None
} 

fn find_char(arr: &[char], target: char) -> Option<usize> {
    for (index, &value) in arr.iter().enumerate() {
        if value == target {
            return Some(index);
        }
    }
    None
} 

fn main_string() {
    let str_arr = ["apple", "banana", "cherry"];
    let search_str = "banana";
    let result = find_string(&str_arr, search_str);
    match result {
        Some(index) => println!("String found at index: {}", index),
        None => println!("String not found"),
    }
}

fn main_float() {
    let float_arr = [1.1, 2.2, 3.3];
    let search_float = 2.2;
    let result = find_float(&float_arr, search_float);
    match result {
        Some(index) => println!("Float found at index: {}", index),
        None => println!("Float not found"),
    }
}

fn main_char() {
    let char_arr = ['a', 'b', 'c'];
    let search_char = 'b';
    let result = find_char(&char_arr, search_char);
    match result {
        Some(index) => println!("Char found at index: {}", index),
        None => println!("Char not found"),
    }
}