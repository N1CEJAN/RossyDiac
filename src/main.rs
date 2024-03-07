use clap::{CommandFactory, Parser, Subcommand};
use clap::error::ErrorKind;

use crate::business::error::ServiceError;
use crate::business::handler::{convert_to_dtp, convert_to_msg};

mod business;

/// A simple-to-use converter prototype.
/// It converts MSG files to DTP files and vice versa.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// The file to convert
    #[arg(short, long)]
    path_to_file: String,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Converts a MSG file to a DTP file
    ConvertToDtp,
    /// Converts a DTP file to a MSG file
    ConvertToMsg,
}

fn main() {
    let cli = Cli::parse();

    println!("Command: {:?}", cli.command);
    println!("File: {:?}", cli.path_to_file);
    let result = match cli.command {
        Command::ConvertToDtp => convert_to_dtp(cli.path_to_file),
        Command::ConvertToMsg => convert_to_msg(cli.path_to_file),
    };

    if let Err(error) = result {
        Cli::command().error(
            match error {
                ServiceError::Io(_) => ErrorKind::Io,
            },
            format!("{}", error),
        ).exit();
    }
}
