use serde::Serialize;
use std::{fs, io::ErrorKind};

use crate::utils;

#[derive(Serialize)]
struct Config {
    app: App,
}

#[derive(Serialize)]
struct App {
    name: String,
}

pub fn init() {
    match fs::create_dir(".cyg") {
        Ok(_) => (),
        Err(err) => {
            if err.kind() == ErrorKind::AlreadyExists {
                println!("error: Cyg already initialized in this repository!");
            } else {
                println!("error: {}", &err);
            }
            return;
        }
    }

    let app_name = utils::prompt::input(&"Your app name?");

    let config = Config {
        app: App { name: app_name },
    };

    let toml = toml::to_string(&config).unwrap();
    let with_comment = format!("# This file is automatically generated by cyg.\n# It is not intended for manual editing.\n\n{}",
        toml
    );

    match fs::write(".cyg/cyg.toml", with_comment) {
        Ok(_) => (),
        Err(err) => println!("error: {}", err),
    }
}
