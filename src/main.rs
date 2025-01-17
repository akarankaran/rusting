use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    field1: Option<String>,
    field2: Option<i32>,
    field3: Option<bool>,
}

impl Config {
    fn default() -> Self {
        Config {
            field1: Some("default_value".to_string()),
            field2: Some(42),
            field3: Some(true),
        }
    }

    fn with_defaults(self) -> Self {
        Config