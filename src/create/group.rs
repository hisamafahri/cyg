use std::fs;

use crate::{utils, model};

pub fn group() {
    utils::check::cyg();
    let raw_config = utils::check::config();
    let parsed_config = utils::config::parse(&raw_config);

    let mut group_list = parsed_config.group;

    let group_name = utils::prompt::input(&"Your group name?");

    group_list.insert( &group_name, model::Group { users: vec![], files: vec![], });
    let config = model::Config {
        app: model::App {name: parsed_config.app.name},
        group: group_list,
    };

    println!("{:?}", config);
    let toml = toml::to_string(&config).unwrap();

    match fs::write(".cyg/cyg.toml", toml) {
        Ok(_) => (),
        Err(err) => println!("error: {}", err),
    }
}
