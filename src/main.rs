use std::cmp::{max, min};

fn main() {
    let array1 = [3, 5, 1, 4, 2];
    let array2 = [-2, -1, -3, -4, -5];
    let array3 = [10];
    let array4: [i32; 0] = []; 

    let (max1, min1) = find_max_min(&array1);
    let (max2, min2) = find_max_min(&array2);
    let (max3, min3) = find_max_min(&array3);
    let (max4, min4) = find_max_min(&array4);

    println!("Array1 - Max: {}, Min: {}", max1, min1);
    println!("Array2 - Max: {}, Min: {}", max2, min2);
    println!("Array3 - Max: {}, Min: {}", max3, min3);
    if !array4.is_empty() {
        println!("Array4 - Max: {}, Min: {}", max4, min4);
    } else {
        println!("Array4 is empty.");
    }
}

fn find_max_min(arr: &[i32]) -> (i32, i32) {
    if arr.is_empty() {
        panic!("Array is empty");
    }
    let mut max_elem = arr[0];
    let mut min_elem = arr[0];
    for &value in arr.iter() {
        max_elem = max(max_elem, value);
        min_elem = min(min_elem, value);
    }
    (max_elem, min_elem)
}