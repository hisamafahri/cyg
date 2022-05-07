use crate::utils;
use std::fs;

pub fn unlock() {
    let path = utils::prompt::input(&"Which file you want to unlock?");

    let ext = &path[path.len() - 4..path.len()];

    if ext != ".cyg" {
        println!("error: not a valid .cyg file!");
        return;
    }

    let original_path = &path[0..path.len() - 4];
    let base_cmd = String::from("gpg");
    let args_cmd = [
        String::from("--output"),
        String::from(original_path),
        String::from("--decrypt"),
        String::from(&path),
    ];
    let unlock_result = utils::cmd::run(&base_cmd, &args_cmd);
    println!("{:?}", unlock_result);

    let rm_result = fs::remove_file(&path);
    println!("{:?}", rm_result);
}
