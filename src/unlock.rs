use crate::utils;

pub fn unlock() {
    let path = utils::prompt::input(&"Which file you want to unlock?");

    // TODO: Check if the extension is .cyg
    let original_path = &path[0..path.len() - 4];
    let base_cmd = String::from("gpg");
    let args_cmd = [
        String::from("--output"),
        String::from(original_path),
        String::from("--decrypt"),
        String::from(path),
    ];
    let result = utils::cmd::run(&base_cmd, &args_cmd);
    println!("{:?}", result)
}
