use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let file_path = "example.txt";
    let content = read_file(file_path)?;
    println!("{}", content);
    Ok(())
}

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}