fn main() {
    let vector = vec![1, 3, 5, 2, 4];
    let max_value = vector.iter().cloned().max().unwrap();
    println!("The maximum value is: {}", max_value);
    
    let empty_vector: Vec<i32> = Vec::new();
    let max_empty = empty_vector.iter().cloned().max().unwrap_or(0);
    println!("The maximum value in empty vector is: {}", max_empty);
    
    let negative_vector = vec![-1, -2, -3, -4, -5];
    let max_negative = negative_vector.iter().cloned().max().unwrap();
    println!("The maximum value in negative vector is: {}", max_negative);

    let mix_vector = vec![1, 2, -3, 4, -5];
    let max_mixed = mix_vector.iter().cloned().max().unwrap();
    println!("The maximum value in mixed vector is: {}", max_mixed);
    
    let large_vector: Vec<i32> = (0..1000).collect();
    let max_large = large_vector.iter().cloned().max().unwrap();
    println!("The maximum value in large vector is: {}", max_large);
    
    let duplicate_vector = vec![1, 2, 3, 2, 1];
    let max_duplicates = duplicate_vector.iter().cloned().max().unwrap();
    println!("The maximum value in duplicate vector is: {}", max_duplicates);
    
    let float_vector = vec![1.1, 2.2, 3.3, 2.2, 1.1];
    let max_float = float_vector.iter().cloned().fold(f32::MIN, f32::max);
    println!("The maximum value in float vector is: {}", max_float);
    
    let large_float_vector: Vec<f64> = (0..1000).map(|x| x as f64 * 1.1).collect();
    let max_large_float = large_float_vector.iter().cloned().max().unwrap();
    println!("The maximum value in large float vector is: {}", max_large_float);
}