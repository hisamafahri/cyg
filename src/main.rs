use clap::Parser;
mod cmd;
mod utils;

mod init;
mod lock;
mod unlock;

fn main() {
    let cmd = cmd::Cmd::parse();

    match &cmd.arg {
        cmd::Arg::Lock => lock::lock(),
        cmd::Arg::Unlock => unlock::unlock(),
        cmd::Arg::Init => init::init(),
    }
}
