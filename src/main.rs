fn sum_slice(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    let result = sum_slice(slice);
    println!("The sum of the slice is: {}", result);

    let arr2 = [10, 20, 30, 40, 50];
    let slice2 = &arr2[0..2];
    let result2 = sum_slice(slice2);
    println!("The sum of the second slice is: {}", result2);

    let arr3: [i32; 0] = [];
    let slice3 = &arr3[0..0];
    let result3 = sum_slice(slice3);
    println!("The sum of the empty slice is: {}", result3);

    let arr4 = [5, 10, 15, 20, 25, 30];
    let slice4 = &arr4[2..];
    let result4 = sum_slice(slice4);
    println!("The sum of the last part of the array is: {}", result4);
}