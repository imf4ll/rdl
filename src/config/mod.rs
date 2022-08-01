use std::fs::read_to_string;
use std::env;

use serde::Deserialize;
use serde_json;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub path: String,
}

pub fn parse() -> Config {
    let path = match env::var("HOME") {
        Ok(string) => string,
        Err(_) => return Config {
            path: String::from(""),
        },
    };

    let content: String = match read_to_string(&format!("{}/.config/rdl/config.json", path)) {
        Ok(txt) => txt,
        Err(_) => return Config {
            path: String::from(""),
        },
    };

    serde_json::from_str::<Config>(&content)
        .expect("Failed to read config file")
}
