mod functions;
mod structures;

use std::io::Result;

use clap::Parser;

use crate::{functions::{clear, create, delete, rename}, structures::{Cli, Command}};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Clear { path } => clear(path)?,
        Command::Create { path } => create(path)?,
        Command::Delete { directory, path, recurse } => delete(directory, path, recurse)?,
        Command::Rename { new, old } => rename(new, old)?
    }

    Ok(())
}