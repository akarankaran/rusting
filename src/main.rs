fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let transformed: Vec<i32> = numbers.iter().map(|&x| match x % 2 {
        1 => x * 2,
        _ => x,
    }).collect();
    
    for number in transformed {
        println!("{}", number);
    }
}