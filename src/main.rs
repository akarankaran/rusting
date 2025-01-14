use std::env;

fn get_env_var(key: &str) -> Option<String> {
    env::var(key).ok()
}

fn main() {
    let db_user = get_env_var("DB_USER").unwrap_or_else(|| "default_user".to_string());
    let db_password = get_env_var("DB_PASSWORD").unwrap_or_else(|| "default_password".to_string());
    let db_host = get_env_var("DB_HOST").unwrap_or_else(|| "localhost".to_string());
    let db_port = get_env_var("DB_PORT").unwrap_or_else(|| "5432".to_string());
    let api_key = get_env_var("API_KEY").unwrap_or_else(|| "no_api_key".to_string());
    
    println!("Database User: {}", db_user);
    println!("Database Password: {}", db_password);
    println!("Database Host: {}", db_host);
    println!("Database Port: {}", db_port);
    println!("API Key: {}", api_key);

    let feature_flag = get_env_var("FEATURE_FLAG").unwrap_or_else(|| "false".to_string());
    if feature_flag == "true" {
        println!("Feature is enabled");
    } else {
        println!("Feature is disabled");
    }

    let timeout = get_env_var("TIMEOUT").map(|s| s.parse::<u64>().unwrap_or(30)).unwrap_or(30);
    println!("Timeout is set to {} seconds", timeout);
}