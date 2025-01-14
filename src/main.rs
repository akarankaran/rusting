use std::fs::File;
use std::io::{self, Read};

fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "example.txt";
    match read_file_content(file_path) {
        Ok(content) => println!("File Content:\n{}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
    Ok(())
}