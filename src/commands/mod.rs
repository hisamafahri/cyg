use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "cyg")]
#[clap(author = "Hisam Fahri <me@hisamafahri.com>")]
#[clap(version = "0.1.0")]
#[clap(about = "Secure your files in your repository")]
#[clap(long_about = "Cyg helps you to secure your files directly inside your repository.")]

pub struct Commands {
    #[clap(subcommand)]
    pub argument: Argument,
}

#[derive(Subcommand)]
pub enum Argument {}
