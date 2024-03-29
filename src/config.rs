//This is primarily a proof of concept
//todo:
//- path override in args
//- let it generate a config in .config
//- handle missings/defaults
//- maybe add one or two more things for it to do
use std::fs;
use std::path::PathBuf;
use dirs::config_dir;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub default_output: String
}

impl Config {
    pub fn new(verbose: bool, config: Option<PathBuf>) -> Config {
        println!("{:?}", config);
        let n = match config {
            Some(path) => {
                let mut p = std::path::PathBuf::new();
                p.push(path);
                p.push("config.json");
                println!("{:?}", p);
                p
            },
            None => {
                if let None = config_dir() {
                    if verbose {println!("Could not find config file, defaulting")}
                    return Config::default();
                };
                config_dir().unwrap()
            }
        };

        let file = fs::File::open(n);
        if let Result::Err(_) = file {
            if verbose {println!("Could not open config file, defaulting")}
            return Config::default()
        };
        let file = file.unwrap();

        let config: Config = serde_json::from_reader(file).unwrap_or_default();
        if verbose {println!("Loaded Config: {:?}", config)}
        config
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            default_output: "./solved_file_default.csv".to_string()
        }
    }
}
