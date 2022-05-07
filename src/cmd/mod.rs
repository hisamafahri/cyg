use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "cyg")]
#[clap(author = "Hisam Fahri <me@hisamafahri.com>")]
#[clap(version = "0.1.0")]
#[clap(about = "Secure your files in your repository")]
#[clap(long_about = "Cyg helps you to secure your files directly inside your repository")]
#[clap(propagate_version = true)]

pub struct Cmd {
    #[clap(subcommand)]
    pub arg: Arg,
}

#[derive(Subcommand)]
pub enum Arg {
    /// Secure files
    #[clap(alias = "l")]
    Lock,
}
