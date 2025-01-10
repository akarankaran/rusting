fn main() {
    let r;
    {
        let x = 42;
        r = &x;
    }
    // println!("{}", r); // This line would cause a compile-time error due to dangling reference

    let y = String::from("Hello");
    let r1 = &y;
    let r2 = &y;
    println!("{}, {}", r1, r2);

    let mut z = String::from("World");
    let r3 = &mut z;
    r3.push_str("!");
    println!("{}", r3);

    let s1 = String::from("Rust");
    let s2 = takes_ownership(s1);
    // println!("{}", s1); // This line would cause a compile-time error because s1's ownership has been moved

    let s3 = String::from("Programming");
    let length = calculate_length(&s3);
    println!("The length of '{}' is {}.", s3, length);
}

fn takes_ownership(s: String) -> String {
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}