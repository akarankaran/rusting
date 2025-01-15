fn main() {
    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    let merged: Vec<i32> = array1.iter().cloned().chain(array2.iter().cloned()).collect();
    println!("{:?}", merged);
    
    let array1 = ["apple", "banana"];
    let array2 = ["orange", "pear"];
    let merged_fruits: Vec<&str> = array1.iter().cloned().chain(array2.iter().cloned()).collect();
    println!("{:?}", merged_fruits);

    let array1 = [true, false];
    let array2 = [false, true];
    let merged_bools: Vec<bool> = array1.iter().cloned().chain(array2.iter().cloned()).collect();
    println!("{:?}", merged_bools);

    let mut array1 = vec![1, 2, 3];
    let array2 = vec![4, 5, 6];
    array1.extend(array2);
    println!("{:?}", array1);
    
    let array1 = [1.1, 2.2, 3.3];
    let array2 = [4.4, 5.5];
    let merged_floats: Vec<f64> = array1.iter().cloned().chain(array2.iter().cloned()).collect();
    println!("{:?}", merged_floats);
    
    let array1 = ['a', 'b'];
    let array2 = ['c', 'd', 'e'];
    let merged_chars: Vec<char> = array1.iter().cloned().chain(array2.iter().cloned()).collect();
    println!("{:?}", merged_chars);
    
    let array1: [i32; 0] = [];
    let array2 = [7, 8, 9];
    let merged_empty: Vec<i32> = array1.iter().cloned().chain(array2.iter().cloned()).collect();
    println!("{:?}", merged_empty);
    
    let array1 = ["rust", "is"];
    let array2 = ["awesome"];
    let merged_strings: Vec<&str> = array1.iter().cloned().chain(array2.iter().cloned()).collect();
    println!("{:?}", merged_strings);
}