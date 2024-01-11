use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub default_output: String
}

impl Config {
    pub fn new() -> Config {
        let file = fs::File::open("config.json").unwrap();
        let p: Config = serde_json::from_reader(file).unwrap();
        p
    }
}
