use crate::commands::EnkiCommands;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "enki", version, about = "Enki Container Runtime")]
pub struct Cli {
    #[command(subcommand)]
    pub command: EnkiCommands,
}
