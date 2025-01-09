fn main() {
    let a = 56;
    let b = 98;
    let gcd = gcd(a, b);
    println!("The GCD of {} and {} is {}", a, b, gcd);
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}