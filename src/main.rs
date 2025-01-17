fn main() {
    let vec_of_tuples = vec![
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
    ];

    for (num, word) in vec_of_tuples {
        println!("Number: {}, Word: {}", num, word);
    }

    let mixed_tuples = vec![
        (1, "apple", true),
        (2, "banana", false),
        (3, "cherry", true),
    ];

    for (id, fruit, is_fruit) in mixed_tuples {
        if is_fruit {
            println!("ID: {}, Fruit: {}", id, fruit);
        }
    }

    let nested_tuples = vec![
        ((1, 2), "pair1"),
        ((3, 4), "pair2"),
    ];

    for ((a, b), label) in nested_tuples {
        println!("Tuple: ({}, {}), Label: {}", a, b, label);
    }

    let processed_tuples: Vec<(i32, String)> = vec![
        (1, "first".to_string()),
        (2, "second".to_string()),
        (3, "third".to_string()),
    ];

    for (index, text) in processed_tuples.iter().enumerate() {
        println!("Index: {}, Text: {}", index, text);
    }

    let transformed_tuples: Vec<(_, _)> = vec![
        (1.5, "one point five"),
        (2.5, "two point five"),
    ];

    for (num, description) in transformed_tuples.iter() {
        println!("Float: {}, Description: {}", num, description);
    }
}