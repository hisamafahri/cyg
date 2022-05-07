use clap::Parser;
mod cmd;

mod lock;

fn main() {
    let cmd = cmd::Cmd::parse();

    match &cmd.arg {
        cmd::Arg::Lock => lock::lock(),
    }
}
