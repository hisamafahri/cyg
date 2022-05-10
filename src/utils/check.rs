use std::path::Path;
use std::{fs, process};

pub fn cyg() {
    // TODO: Check if GPG cli is exist
    match Path::new(".cyg").exists() {
        true => (),
        false => {
            println!(
                "error: Cyg is not initialized in this repository!
Please run 'cyg init' to get started."
            );
            return;
        }
    }
    match Path::new(".cyg/cyg.toml").exists() {
        true => (),
        false => {
            println!("error: Configuration file could not been found!");
            return;
        }
    }
}

pub fn config() -> String {
    let contents = match fs::read_to_string(".cyg/cyg.toml") {
        Ok(c) => c,
        Err(_) => {
            println!("error: Error while reading the configuration file");
            process::exit(1);
        }
    };

    return contents;
  }
