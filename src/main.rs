fn main() {
    let tuple1 = (1, "Hello", 3.14);
    match tuple1 {
        (x, y, z) => {
            println!("Integer: {}, String: {}, Float: {}", x, y, z);
        }
    }

    let tuple2 = (true, 'R', 42);
    match tuple2 {
        (a, b, c) => {
            println!("Boolean: {}, Char: {}, Number: {}", a, b, c);
        }
    }

    let tuple3 = (String::from("Rust"), 7);
    match tuple3 {
        (s, n) => {
            println!("String: {}, Integer: {}", s, n);
        }
    }

    let tuple4 = (0.5, "Match", true);
    match tuple4 {
        (f, str, b) => {
            println!("Float: {}, String: {}, Boolean: {}", f, str, b);
        }
    }

    let tuple5 = (10, 20, 30);
    match tuple5 {
        (a, b, c) if a + b + c > 50 => {
            println!("Sum is greater than 50: {}", a + b + c);
        }
        (a, b, c) => {
            println!("Sum is 50 or less: {}", a + b + c);
        }
    }
}