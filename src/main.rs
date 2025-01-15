fn is_sorted_ascending(arr: &[i32]) -> bool {
    for i in 0..arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    true
}

fn main() {
    let array1 = [1, 2, 3, 4, 5];
    let array2 = [5, 4, 3, 2, 1];
    let array3 = [1, 2, 2, 3, 4];
    let array4 = [];
    
    println!("{}", is_sorted_ascending(&array1)); // true
    println!("{}", is_sorted_ascending(&array2)); // false
    println!("{}", is_sorted_ascending(&array3)); // true
    println!("{}", is_sorted_ascending(&array4)); // true
}