fn main() {
    let str1 = String::from("Hello, ");
    let str2 = String::from("world!");
    let result = concatenate(&str1, &str2);
    println!("{}", result);
}

fn concatenate(s1: &str, s2: &str) -> String {
    let mut concat = String::new();
    concat.push_str(s1);
    concat.push_str(s2);
    concat
}