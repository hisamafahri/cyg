use std::{fs, process};

use crate::utils;
use crate::model;

pub fn group() {
    utils::check::cyg();
    let contents = utils::check::config();

    // TODO: Separate parse functinos
    let config_data: model::Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            println!("error: Unable to parse data from the configuration file");
            process::exit(1);
        }
    };

    let mut group_list = config_data.group;

    let group_name = utils::prompt::input(&"Your group name?");

    group_list.insert(
        &group_name,
        model::Group {
            users: vec![],
            files: vec![],
        });
    let config = model::Config {
        app: model::App {name: config_data.app.name},
        group: group_list,
    };

    println!("{:?}", config);
    let toml = toml::to_string(&config).unwrap();

    match fs::write(".cyg/cyg.toml", toml) {
        Ok(_) => (),
        Err(err) => println!("error: {}", err),
    }
}
