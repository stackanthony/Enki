use std::process::ExitStatus;

use clap::{Error, Subcommand, ValueEnum};

pub use run::execute_run;
pub mod run;

#[derive(Subcommand, Debug)]
pub enum EnkiCommands {
    Run {
        image_path: String,
        entry_point: String,
    },
}

pub fn handle_command(command: EnkiCommands) -> Result<ExitStatus, std::io::Error> {
    match command {
        EnkiCommands::Run {
            image_path,
            entry_point,
        } => run::execute_run(&image_path, &entry_point),
    }
}
