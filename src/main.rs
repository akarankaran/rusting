fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = String::from(" How are you?");
    
    let combined = combine_strings(&s1, &s2, &s3);
    println!("{}", combined);
}

fn combine_strings<'a>(s1: &'a str, s2: &'a str, s3: &'a str) -> String {
    format!("{}{}{}", s1, s2, s3)
}