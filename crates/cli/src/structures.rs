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
        path: String
    },
    Create {
        path: String
    },
    Rename {
        new: String,
        old: String
    }
}