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
    let base_cmd = String::from("ls");
    let args_cmd = [String::from("-a")];
    let result = utils::cmd::run(&base_cmd, &args_cmd);
    println!("{:?}", result)
}

// gpg --output encrypted.data --symmetric --cipher-algo AES256 un_encrypted.data
