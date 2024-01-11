//This is primarily a proof of concept

//todo:
//- path override in args
//- let it generate a config in .config
//- handle missings/defaults
//- maybe add one or two more things for it to do
use std::fs;
use dirs::config_dir;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub default_output: String
}

impl Config {
    pub fn new(/*verbose: bool, config: Option<String>*/) -> Config {
        let n = config_dir(); 
        if let None = n {
            return Config::default();
        };
        let mut n = n.unwrap();
        n.push("sudoku_solve/config.json");

        let file = fs::File::open(n);
        if let Result::Err(_) = file {
            return Config::default()
        };
        let file = file.unwrap();

        let config: Config = serde_json::from_reader(file).unwrap_or_default();
        config
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            default_output: "./config.json".to_string()
        }
    }
}
