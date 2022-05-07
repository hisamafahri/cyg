use std::{io, process::Command};

pub fn run(base: &str, args: &[String]) -> Result<std::process::Output, io::Error> {
    return Command::new(base)
        .args(args)
        .output()
        .and_then(|r| match r.status.success() {
            true => Ok(r),
            false => Err(io::Error::new(io::ErrorKind::InvalidData, "Process error")),
        });
}
