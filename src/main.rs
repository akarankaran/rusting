fn rotate_left<T>(arr: &mut Vec<T>) {
    if !arr.is_empty() {
        let first = arr.remove(0);
        arr.push(first);
    }
}

fn main() {
    let mut array1 = vec![1, 2, 3, 4, 5];
    rotate_left(&mut array1);
    println!("{:?}", array1);

    let mut array2 = vec!['a', 'b', 'c', 'd'];
    rotate_left(&mut array2);
    println!("{:?}", array2);

    let mut array3: Vec<i32> = vec![];
    rotate_left(&mut array3);
    println!("{:?}", array3);

    let mut array4 = vec![true, false, true];
    rotate_left(&mut array4);
    println!("{:?}", array4);

    let mut array5 = vec![10.1, 20.2, 30.3];
    rotate_left(&mut array5);
    println!("{:?}", array5);
}