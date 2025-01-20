fn print_display<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}

fn main() {
    let number = 42;
    let text = "Hello, Rust!";
    let float = 3.14;

    print_display(number);
    print_display(text);
    print_display(float);

    let vec = vec![1, 2, 3];
    print_display(vec.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "));

    let tuple = (1, "tuple", 3.0);
    print_display(format!("{:?}", tuple));
}