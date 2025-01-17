use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Cargo {
    manifest: Manifest,
}

#[derive(Debug, Serialize, Deserialize)]
struct Manifest {
    package: Package,
}

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}

fn main() {
    let data = r#"
    {
        "manifest": {
            "package": {
                "name": "example_crate",
                "version": "0.1.0",
                "authors": ["Alice", "Bob"]
            }
        }
    }"#;

    let cargo: Cargo = serde_json::from_str(data).unwrap();
    println!("Package Name: {}", cargo.manifest.package.name);
    println!("Package Version: {}", cargo.manifest.package.version);
    for author in &cargo.manifest.package.authors {
        println!("Author: {}", author);
    }
}