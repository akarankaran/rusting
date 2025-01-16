fn main() {
    let result: Result<i32, Box<dyn std::error::Error>> = do_something();

    match result {
        Ok(value) => println!("Success: {}", value),
        Err(e) => match e.downcast_ref::<std::io::Error>() {
            Some(io_error) => println!("IO Error: {}", io_error),
            None => match e.downcast_ref::<std::num::ParseIntError>() {
                Some(parse_error) => println!("Parse Error: {}", parse_error),
                None => match e.downcast_ref::<String>() {
                    Some(string_error) => println!("String Error: {}", string_error),
                    None => println!("Unknown Error: {}", e),
                },
            },
        },
    }
}

fn do_something() -> Result<i32, Box<dyn std::error::Error>> {
    let num: &str = "42a";
    let parsed: i32 = num.parse()?;
    Ok(parsed)
}