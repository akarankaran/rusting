use std::vec::Vec;

fn main() {
    let array: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    for row in array.iter() {
        for &element in row.iter() {
            println!("{}", element);
        }
    }
}