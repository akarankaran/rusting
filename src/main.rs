use std::num::ParseFloatError;

fn safe_divide(num: f64, denom: f64) -> Result<f64, &'static str> {
    if denom == 0.0 {
        Err("Division by zero")
    } else {
        Ok(num / denom)
    }
}

fn parse_and_divide(num_str: &str, denom_str: &str) -> Result<f64, ParseFloatError> {
    let num: f64 = num_str.parse()?;
    let denom: f64 = denom_str.parse()?;
    safe_divide(num, denom)
}

fn main() {
    let inputs = vec![("10", "2"), ("10", "0"), ("abc", "2"), ("10", "5")];

    for (num_str, denom_str) in inputs {
        match parse_and_divide(num_str, denom_str) {
            Ok(result) => println!("Result of {} / {} = {}", num_str, denom_str, result),
            Err(e) => println!("Error: {}", e),
        }
    }
}