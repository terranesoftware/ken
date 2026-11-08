mod functions;
mod structures;

use std::io::Result;

use clap::Parser;

use crate::{functions::{clear, create}, structures::{Cli, Command}};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Clear { path } => clear(path)?,
        Command::Create { path } => create(path)?
    }

    Ok(())
}