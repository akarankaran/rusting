fn swap_if_too_small(tuple: (i32, i32)) -> (i32, i32) {
    if tuple.0 < 10 && tuple.1 < 10 {
        (tuple.1, tuple.0)
    } else {
        tuple
    }
}

fn main() {
    let tuples = vec![(5, 3), (12, 8), (7, 20), (9, 1), (15, 5)];
    let swapped_tuples: Vec<(i32, i32)> = tuples.into_iter().map(swap_if_too_small).collect();
    for t in swapped_tuples {
        println!("{:?}", t);
    }
}