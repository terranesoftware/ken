use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about, bin_name = "ken", name = "ken", version)]
pub(super) struct Cli {
    #[command(subcommand)]
    pub(super) command: Command
}

#[derive(Subcommand)]
pub(super) enum Command {
    Clear {
        #[arg(long, short)]
        directory: bool,
        
        path: PathBuf
    },
    Create {
        #[arg(long, requires = "directory", short)]
        all: bool,
        
        #[arg(long, short)]
        directory: bool,
        
        path: PathBuf
    },
    Delete {
        #[arg(long, short)]
        directory: bool,
        
        path: PathBuf,

        #[arg(long, requires = "directory", short)]
        recurse: bool
    },
    Rename {
        new: PathBuf,
        old: PathBuf
    }
}