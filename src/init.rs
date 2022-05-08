use crate::utils;

// TOML
use serde::Serialize;
use std::{collections::BTreeMap, fs};
use toml;

#[derive(Debug, Default, Serialize)]
struct Config<'a> {
    app: BTreeMap<&'a str, App<'a>>,
}

#[derive(Debug, Serialize)]
struct App<'a> {
    name: &'a str,
}

pub fn init() {
    let app_name = utils::prompt::input(&"Your app name?");

    let d = fs::create_dir(".cyg/");
    println!("{:?}", d);

    let mut config = Config::default();
    config.app.insert("info", App { name: &app_name });

    let toml_string = toml::to_string(&config).expect("Could not encode TOML value");
    println!("{}", toml_string);
    fs::write(".cyg/cyg.toml", toml_string).expect("Could not write to file!");
}
