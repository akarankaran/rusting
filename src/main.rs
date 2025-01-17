use std::vec::Vec;

fn main() {
    let nested_vecs: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![4, 5],
        vec![6, 7, 8, 9],
        vec![],
        vec![10],
    ];

    for (i, inner_vec) in nested_vecs.iter().enumerate() {
        println!("Vector {}: {:?}", i, inner_vec);
        for (j, &element) in inner_vec.iter().enumerate() {
            println!("Element at [{}][{}]: {}", i, j, element);
        }
    }

    let string_vecs: Vec<Vec<&str>> = vec![
        vec!["apple", "banana"],
        vec!["carrot"],
        vec!["date", "elderberry", "fig"],
    ];

    for (i, inner_vec) in string_vecs.iter().enumerate() {
        println!("String Vector {}: {:?}", i, inner_vec);
        for (j, &element) in inner_vec.iter().enumerate() {
            println!("Element at [{}][{}]: {}", i, j, element);
        }
    }

    let mixed_vecs: Vec<Vec<Box<dyn std::fmt::Debug>>> = vec![
        vec![Box::new(1), Box::new("two")],
        vec![Box::new(3.5)],
    ];

    for (i, inner_vec) in mixed_vecs.iter().enumerate() {
        println!("Mixed Vector {}: {:?}", i, inner_vec);
        for (j, element) in inner_vec.iter().enumerate() {
            println!("Element at [{}][{}]: {:?}", i, j, element);
        }
    }
}