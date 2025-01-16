use std::vec::Vec;

fn main() {
    let vec1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let vec2: Vec<i32> = vec![];
    let vec3: Vec<i32> = vec![10, 20, 30];

    check_vector_length(&vec1);
    check_vector_length(&vec2);
    check_vector_length(&vec3);
}

fn check_vector_length(vec: &Vec<i32>) {
    match vec.len() {
        0 => println!("The vector is empty."),
        1 => println!("The vector has one element."),
        2..=10 => println!("The vector has between 2 and 10 elements."),
        _ => println!("The vector has more than 10 elements."),
    }
}