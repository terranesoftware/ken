mod functions;
mod structures;

use std::io::Result;

use clap::Parser;

use crate::{functions::{clear, copy, create, delete, list, r#move, read}, structures::{Cli, Command}};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Clear { directory, path } => clear(directory, path)?,
        Command::Copy { from, to } => copy(from, to)?,
        Command::Create { all, directory, path } => create(all, directory, path)?,
        Command::Delete { directory, path, recurse } => delete(directory, path, recurse)?,
        Command::List { path, recurse } => list(path, recurse)?,
        Command::Move { from, to } => r#move(from, to)?,
        Command::Read { path } => read(path)?
    }

    Ok(())
}