fn main() {
    let float_values: [f32; 5] = [10.5, -20.7, 0.0, 100.9, -300.9];

    for &value in &float_values {
        let int_value = value as i32;
        println!("f32: {}, i32: {}", value, int_value);
    }

    // Additional variations of float values
    let more_float_values: [f32; 5] = [f32::MAX, f32::MIN, 123456.789, -123456.789, 3.14159];

    for &value in &more_float_values {
        let int_value = value as i32;
        println!("f32: {}, i32: {}", value, int_value);
    }
}