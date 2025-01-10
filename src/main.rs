use std::vec::Vec;

fn sort_array(arr: &mut [i32]) {
    arr.sort();
}

fn main() {
    let mut array1 = [5, 3, 8, 1, 2];
    sort_array(&mut array1);
    println!("{:?}", array1);

    let mut array2 = [10, 7, 9, 5, 3];
    sort_array(&mut array2);
    println!("{:?}", array2);

    let mut array3 = [12, 15, 10, 11];
    sort_array(&mut array3);
    println!("{:?}", array3);

    let mut array4 = [1, 0, -1, 3, -3];
    sort_array(&mut array4);
    println!("{:?}", array4);

    let mut array5: Vec<i32> = vec![100, -50, 0, 50, 25];
    sort_array(&mut array5);
    println!("{:?}", array5);
    
    let mut array6 = [3];
    sort_array(&mut array6);
    println!("{:?}", array6);
    
    let mut array7: Vec<i32> = vec![];
    sort_array(&mut array7);
    println!("{:?}", array7);
}