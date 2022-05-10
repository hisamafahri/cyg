use crate::model;
use crate::utils;
use std::fs;
use toml;

pub fn group() {
    utils::check::cyg();

    // TODO: Write to the config file
    // TODO: Make read the current config file as its own function
    let contents = match fs::read_to_string(".cyg/cyg.toml") {
        Ok(c) => c,
        Err(_) => {
            println!("error: Error while reading the configuration file");
            return;
        }
    };

    let data: model::Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            println!("error: Unable to load data from the configuration file");
            return;
        }
    };

    println!("{:?}", data);
}
