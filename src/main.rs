use clap::Parser;
mod cmd;
mod model;
mod utils;

mod create;
mod init;
mod lock;
mod unlock;

fn main() {
    let cmd = cmd::Cmd::parse();

    match &cmd.arg {
        cmd::Arg::Lock => lock::lock(),
        cmd::Arg::Unlock => unlock::unlock(),
        cmd::Arg::Init => init::init(),
        cmd::Arg::Create(resource) => create::create(resource),
    }
}
