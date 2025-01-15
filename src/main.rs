use std::vec::Vec;

fn merge_vectors<T: Clone>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
    let mut merged = vec1.clone();
    merged.extend(vec2.clone());
    merged
}

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let merged_ints = merge_vectors(vec1, vec2);
    
    let vec3 = vec!["apple", "banana"];
    let vec4 = vec!["cherry", "date"];
    let merged_strings = merge_vectors(vec3, vec4);
    
    let vec5 = vec![1.1, 2.2];
    let vec6 = vec![3.3, 4.4];
    let merged_floats = merge_vectors(vec5, vec6);
    
    let vec7 = vec![true, false];
    let vec8 = vec![false, true];
    let merged_bools = merge_vectors(vec7, vec8);
    
    println!("{:?}", merged_ints);
    println!("{:?}", merged_strings);
    println!("{:?}", merged_floats);
    println!("{:?}", merged_bools);
}