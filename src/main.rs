fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    let numbers = vec![(10.0, 2.0), (10.0, 0.0), (15.0, 3.0)];

    for (a, b) in numbers {
        match divide(a, b) {
            Ok(result) => println!("{} / {} = {}", a, b, result),
            Err(e) => println!("Error: {}", e),
        }
    }
}