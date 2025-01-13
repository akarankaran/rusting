fn main() {
    let num = 5;
    let square = calculate_square(num);
    println!("The square of {} is {}", num, square);
}

fn calculate_square(n: i32) -> i32 {
    n * n
}