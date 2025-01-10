fn main() {
    let s1 = String::from("Hello");
    let s2 = takes_ownership(s1);
    let s3 = String::from("World");
    let s4 = gives_back(s3);
    println!("{}, {}", s2, s4);
}

fn takes_ownership(s: String) -> String {
    s
}

fn gives_back(s: String) -> String {
    s
}