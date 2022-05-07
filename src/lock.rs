use crate::utils;

pub fn lock() {
    let path = utils::prompt::input(&"Which file you want to lock?");
    let recipient = utils::prompt::input(&"What is the recipient email?");

    let result_path = format!("{}.cyg", path);
    let base_cmd = String::from("gpg");
    let args_cmd = [
        String::from("--output"),
        String::from(result_path),
        String::from("--encrypt"),
        String::from("--recipient"),
        String::from(recipient),
        String::from(path),
    ];
    let result = utils::cmd::run(&base_cmd, &args_cmd);
    println!("{:?}", result)
}
