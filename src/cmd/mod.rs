use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "cyg")]
#[clap(author = "Hisam Fahri <me@hisamafahri.com>")]
#[clap(version = "0.2.0")]
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
    Lock,

    /// Unlock secured files
    Unlock,

    /// Initialize cyg in current repository
    Init,

    /// Create a new group or user to the configuration
    Create(Resource),
}

#[derive(Args, Debug)]
pub struct Resource {
    /// Choose between a 'group' or 'user'
    pub resource: String,
}
