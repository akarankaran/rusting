use std::collections::HashSet;

fn main() {
    let input_array = vec![1, 2, 3, 3, 4, 5, 2, 1];
    let unique_array: Vec<i32> = input_array.into_iter().collect::<HashSet<_>>().into_iter().collect();
    println!("{:?}", unique_array);

    let input_array2 = vec!["apple", "banana", "apple", "orange", "banana"];
    let unique_array2: Vec<&str> = input_array2.into_iter().collect::<HashSet<_>>().into_iter().collect();
    println!("{:?}", unique_array2);

    let input_array3 = vec![true, false, true, true, false];
    let unique_array3: Vec<bool> = input_array3.into_iter().collect::<HashSet<_>>().into_iter().collect();
    println!("{:?}", unique_array3);
    
    let input_array4: Vec<char> = vec!['a', 'b', 'a', 'c', 'b', 'd'];
    let unique_array4: Vec<char> = input_array4.into_iter().collect::<HashSet<_>>().into_iter().collect();
    println!("{:?}", unique_array4);

    let input_array5: Vec<f64> = vec![3.14, 2.71, 3.14, 1.41, 2.71];
    let unique_array5: Vec<f64> = input_array5.into_iter().collect::<HashSet<_>>().into_iter().collect();
    println!("{:?}", unique_array5);

    let input_array6: Vec<i32> = (1..=10).cycle().take(20).collect();
    let unique_array6: Vec<i32> = input_array6.into_iter().collect::<HashSet<_>>().into_iter().collect();
    println!("{:?}", unique_array6);
}