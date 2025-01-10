fn main() {
    let array = [1, 2, 3, 4, 5];
    let sum = calculate_sum(&array);
    println!("The sum is: {}", sum);
}

fn calculate_sum(arr: &[i32]) -> i32 {
    let mut total = 0;
    for &item in arr.iter() {
        total += item;
    }
    total
}