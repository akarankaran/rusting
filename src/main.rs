fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = outer_function()?;
    println!("Result: {}", result);
    Ok(())
}

fn outer_function() -> Result<i32, Box<dyn std::error::Error>> {
    let value = inner_function()?;
    Ok(value + 10)
}

fn inner_function() -> Result<i32, Box<dyn std::error::Error>> {
    let num: Option<i32> = Some(5);
    let valid_number = num.ok_or("Number was None")?;
    Ok(valid_number * 2)
}