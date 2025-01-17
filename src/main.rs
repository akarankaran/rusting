fn main() {
    let tuples = vec![
        (Some(1), Some(2)),
        (None, Some(3)),
        (Some(4), None),
        (None, None),
    ];

    for tuple in tuples {
        match tuple {
            (Some(a), Some(b)) => println!("Both values: {} and {}", a, b),
            (Some(a), None) => println!("First value: {}", a),
            (None, Some(b)) => println!("Second value: {}", b),
            (None, None) => println!("Both values are None"),
        }
    }
}