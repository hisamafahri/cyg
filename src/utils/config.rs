use std::process;
use crate::model;

pub fn parse(contents: &str) -> model::Config {
    let config_data: model::Config = match toml::from_str(contents) {
        Ok(d) => d,
        Err(_) => {
            println!("error: Unable to parse data from the configuration file");
            process::exit(1);
        }
    };

    return config_data;
}
