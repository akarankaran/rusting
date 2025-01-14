use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

fn create_file<P: AsRef<Path>>(path: P) -> Result<File, io::Error> {
    let file = File::create(path)?;
    Ok(file)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "output.txt";
    
    match create_file(file_path) {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "Hello, World!") {
                eprintln!("Failed to write to file: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            return Err(Box::new(e));
        }
    }

    if let Err(e) = create_file("") {
        eprintln!("Invalid file path: {}", e);
    }

    if let Err(e) = create_file("/invalid/path/to/file.txt") {
        eprintln!("Error creating file in invalid path: {}", e);
    }

    Ok(())
}