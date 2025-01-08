fn main() {
    // Checking maximum and minimum values for various integer types
    let i8_min = std::i8::MIN;
    let i8_max = std::i8::MAX;

    let i16_min = std::i16::MIN;
    let i16_max = std::i16::MAX;

    let i32_min = std::i32::MIN;
    let i32_max = std::i32::MAX;

    let i64_min = std::i64::MIN;
    let i64_max = std::i64::MAX;

    let i128_min = std::i128::MIN;
    let i128_max = std::i128::MAX;

    let isize_min = std::isize::MIN;
    let isize_max = std::isize::MAX;

    // Print the results
    println!("i8: min = {}, max = {}", i8_min, i8_max);
    println!("i16: min = {}, max = {}", i16_min, i16_max);
    println!("i32: min = {}, max = {}", i32_min, i32_max);
    println!("i64: min = {}, max = {}", i64_min, i64_max);
    println!("i128: min = {}, max = {}", i128_min, i128_max);
    println!("isize: min = {}, max = {}", isize_min, isize_max);
}