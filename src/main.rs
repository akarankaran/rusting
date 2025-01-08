fn main() {
    let mut a = 5;
    let mut b = 10;

    a = a + b;
    b = a - b;
    a = a - b;

    println!("a: {}, b: {}", a, b);

    let mut x = 15.5;
    let mut y = 20.8;

    x = x + y;
    y = x - y;
    x = x - y;

    println!("x: {}, y: {}", x, y);

    let mut p = 'A';
    let mut q = 'Z';

    p = (p as u8) ^ (q as u8) as char;
    q = (p as u8) ^ (q as u8) as char;
    p = (p as u8) ^ (q as u8) as char;

    println!("p: {}, q: {}", p, q);
}