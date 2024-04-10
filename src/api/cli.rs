use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, Subcommand};
use log::debug;

use crate::business::error::ServiceError;
use crate::business::handler::{convert_to_dtp, convert_to_msg};

/// A simple-to-use converter prototype.
/// It converts MSG files to DTP files and vice versa.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Converts a MSG file to a DTP file
    ConvertToDtp {
        /// The file to convert
        #[arg(short = 'f', long = "file")]
        path_to_msg_file: String,
    },
    /// Converts a DTP file to a MSG file
    ConvertToMsg {
        /// The file to convert
        #[arg(short = 'f', long = "file")]
        path_to_dtp_file: String,
    },
}

pub fn run() {
    let cli = Cli::parse();

    debug!("Command: {:?}", cli.command);
    let result = match cli.command {
        Command::ConvertToDtp { path_to_msg_file } => convert_to_dtp(path_to_msg_file.as_str()),
        Command::ConvertToMsg { path_to_dtp_file } => convert_to_msg(path_to_dtp_file.as_str()),
    };

    if let Err(error) = result {
        Cli::command()
            .error(
                match error {
                    ServiceError::Io(_) => ErrorKind::Io,
                    ServiceError::Parser(_) => ErrorKind::Format,
                },
                format!("{}", error),
            )
            .exit();
    }
}
