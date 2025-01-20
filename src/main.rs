use serde::{Serialize, Deserialize};
use serde_json::{self, Result};

trait JsonConvertible {
    fn to_json(&self) -> Result<String>;
}

#[derive(Serialize, Deserialize)]
struct Cargo {
    name: String,
    version: String,
}

impl JsonConvertible for Cargo {
    fn to_json(&self) -> Result<String> {
        serde_json::to_string(self)
    }
}

fn main() {
    let cargo = Cargo {
        name: String::from("example-crate"),
        version: String::from("0.1.0"),
    };

    match cargo.to_json() {
        Ok(json) => println!("Cargo JSON: {}", json),
        Err(e) => println!("Failed to convert to JSON: {}", e),
    }
}