use clap::Parser;
mod cmd;
mod utils;

fn main() {
    let cmd = cmd::Cmd::parse();

    match &cmd.arg {
        cmd::Arg::Lock => lock(),
    }
}

fn lock() {
    let path = utils::prompt::input(&"Which file you want to secure? ");
    let base_cmd = String::from("gpg");
    let args_cmd = [
        String::from("--output"),
        String::from("test.cyg"),
        String::from("--encrypt"),
        String::from("--recipient"),
        String::from("me@hisamafahri.com"),
        String::from(path),
    ];
    let result = utils::cmd::run(&base_cmd, &args_cmd);
    println!("{:?}", result)
}
