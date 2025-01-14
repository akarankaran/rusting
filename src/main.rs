fn main() {
    let value: Option<i32> = None;
    let default_value = 10;
    let result = value.unwrap_or(default_value);
    println!("{}", result);

    let value2: Option<&str> = Some("Hello");
    let default_str = "Default";
    let result2 = value2.unwrap_or(default_str);
    println!("{}", result2);

    let value3: Option<i32> = Some(5);
    let result3 = value3.unwrap_or_else(|| {
        println!("Using default value");
        20
    });
    println!("{}", result3);

    let config: Option<String> = None;
    let default_config = "config.toml".to_string();
    let config_file = config.as_ref().unwrap_or(&default_config);
    println!("{}", config_file);

    let possibly_none: Option<f64> = Some(3.14);
    let fallback = 2.71;
    let final_value = possibly_none.unwrap_or(fallback);
    println!("{}", final_value);
}