use clap::Parser;
mod commands;

fn main() {
    commands::Commands::parse();
}
