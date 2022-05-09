use std::path::Path;

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
