fn is_sorted(arr: &[i32]) -> bool {
    for i in 1..arr.len() {
        if arr[i] < arr[i - 1] {
            return false;
        }
    }
    true
}

fn main() {
    let array1 = [1, 2, 3, 4, 5];
    let array2 = [5, 4, 3, 2, 1];
    let array3 = [1, 2, 2, 3, 4];
    let array4: [i32; 0] = [];

    println!("{}", is_sorted(&array1));
    println!("{}", is_sorted(&array2));
    println!("{}", is_sorted(&array3));
    println!("{}", is_sorted(&array4));
}