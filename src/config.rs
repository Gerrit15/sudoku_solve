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
    pub fn new() -> Config {
        let mut n = config_dir().unwrap();
        n.push("sudoku_solve/config.json");
        let file = fs::File::open(n).unwrap();
        let p: Config = serde_json::from_reader(file).unwrap();
        p
    }
}
