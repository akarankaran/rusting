fn main() {
    let a = 10;
    let b = 20;
    let c = 30;

    let (ref x, ref y, ref z) = (&a, &b, &c);

    println!("x: {}, y: {}, z: {}", x, y, z);

    let d = 40;
    let e = 50;

    let (ref p1, ref p2) = (&d, &e);
    
    println!("p1: {}, p2: {}", p1, p2);
    
    let x2 = 5;
    let y2 = 15;

    let (ref a2, ref b2) = if true { (&x2, &y2) } else { (&x2, &y2) };
    
    println!("a2: {}, b2: {}", a2, b2);
}