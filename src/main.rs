fn main() {
    let array = [1, 2, 3, 4, 5, 6];
    let middle = get_middle(&array);
    println!("{:?}", middle);

    let array_odd = [10, 20, 30, 40, 50];
    let middle_odd = get_middle(&array_odd);
    println!("{:?}", middle_odd);

    let empty_array: [i32; 0] = [];
    let middle_empty = get_middle(&empty_array);
    println!("{:?}", middle_empty);
}

fn get_middle(arr: &[i32]) -> Vec<i32> {
    let len = arr.len();
    if len == 0 {
        return Vec::new();
    }
    if len % 2 == 0 {
        return vec![arr[len / 2 - 1], arr[len / 2]];
    } else {
        return vec![arr[len / 2]];
    }
}