use crate::commands::handle_command;
use clap::Parser;
use cli::Cli;

mod cli;
mod commands;

fn main() {
    let args = Cli::parse();

    println!("args: {:?}", args);
    if let Err(err_str) = handle_command(args.command) {
        panic!("Error: {}", err_str);
    }
}
