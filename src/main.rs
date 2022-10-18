use clap::Parser;
use commands::Commands;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = anyhow::Result<T, Error>;

fn main() -> Result<()> {
    let commands = Commands;
    commands.process(args::Args::parse())
}
