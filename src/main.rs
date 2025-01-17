use std::vec::Vec;

fn main() {
    let coordinates: Vec<(f64, f64)> = vec![
        (1.0, 2.0),
        (3.5, 4.5),
        (6.1, 7.0),
        (0.0, 0.0),
        (8.4, 5.6),
    ];

    for (x, y) in coordinates {
        println!("Coordinate: ({}, {})", x, y);
    }
}