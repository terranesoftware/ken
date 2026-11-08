mod functions;
mod structures;

use std::io::Result;

use clap::Parser;

use crate::{functions::{clear, create, delete, list, read, rename}, structures::{Cli, Command}};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Clear { directory, path } => clear(directory, path)?,
        Command::Create { all, directory, path } => create(all, directory, path)?,
        Command::Delete { directory, path, recurse } => delete(directory, path, recurse)?,
        Command::List { path, recurse } => list(path, recurse)?,
        Command::Read { path } => read(path)?,
        Command::Rename { new, old } => rename(new, old)?
    }

    Ok(())
}