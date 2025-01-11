fn main() {
    let a = 5;
    let b = 10;
    let sum = add(a, b);
    println!("The sum of {} and {} is {}", a, b, sum);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}