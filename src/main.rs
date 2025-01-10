fn main() {
    let a: i32 = 5;
    let b: i32 = a;
    println!("a: {}, b: {}", a, b);

    let x: f64 = 10.5;
    let y: f64 = x;
    println!("x: {}, y: {}", x, y);

    let c: char = 'R';
    let d: char = c;
    println!("c: {}, d: {}", c, d);

    let flag: bool = true;
    let status: bool = flag;
    println!("flag: {}, status: {}", flag, status);

    let large_number: u64 = 1_000_000;
    let another_number: u64 = large_number;
    println!("large_number: {}, another_number: {}", large_number, another_number);

    let small_number: i8 = -128;
    let tiny_number: i8 = small_number;
    println!("small_number: {}, tiny_number: {}", small_number, tiny_number);

    let decimal: f32 = 3.14;
    let another_decimal: f32 = decimal;
    println!("decimal: {}, another_decimal: {}", decimal, another_decimal);

    let byte: u8 = 255;
    let another_byte: u8 = byte;
    println!("byte: {}, another_byte: {}", byte, another_byte);

    let long_integer: isize = 100_000;
    let copy_integer: isize = long_integer;
    println!("long_integer: {}, copy_integer: {}", long_integer, copy_integer);
}