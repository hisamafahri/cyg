use std::collections::BTreeMap;
use std::process;
use std::{fs, io::ErrorKind};

use crate::model;

use crate::utils;

pub fn init() {
    // Create .cyg directory if not exist
    // return error if exist
    match fs::create_dir(".cyg") {
        Ok(_) => (),
        Err(err) => {
            if err.kind() == ErrorKind::AlreadyExists {
                println!("error: Cyg already initialized in this repository!");
            } else {
                println!("error: {}", &err);
            }
            process::exit(1);
        }
    }

    let app_name = utils::prompt::input(&"Your app name?");

    let config = model::Config {
        app: model::App { name: app_name },
        group: BTreeMap::new(),
    };

    let toml = toml::to_string(&config).unwrap();

    match fs::write(".cyg/cyg.toml", toml) {
        Ok(_) => (),
        Err(err) => println!("error: {}", err),
    }
}
