use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, Subcommand};
use log::debug;

use crate::business::error::Error;
use crate::business::handler::*;

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
        /// The directories to search when resolving references
        #[arg(short = 's', long = "source-directories")]
        path_to_source_directories: Vec<String>,
        /// The directory where the conversion result will be written
        #[arg(short = 'd', long = "destination-directory")]
        path_to_destination_directory: String,
    },
    /// Converts a DTP file to a MSG file
    ConvertToMsg {
        /// The file to convert
        #[arg(short = 'f', long = "file")]
        path_to_dtp_file: String,
        /// The directories to search when resolving references
        #[arg(short = 's', long = "source-directories")]
        path_to_source_directories: Vec<String>,
        /// The directory where the conversion result will be written
        #[arg(short = 'd', long = "destination-directory")]
        path_to_destination_directory: String,
    },
}

pub fn run() {
    let cli = Cli::parse();

    debug!("Command: {:?}", cli.command);
    let result = match cli.command {
        Command::ConvertToDtp {
            path_to_msg_file,
            path_to_source_directories,
            path_to_destination_directory,
        } => convert_to_dtp(
            &path_to_msg_file,
            &path_to_source_directories,
            &path_to_destination_directory,
        ),
        Command::ConvertToMsg {
            path_to_dtp_file,
            path_to_source_directories,
            path_to_destination_directory,
        } => convert_to_msg(
            &path_to_dtp_file,
            &path_to_source_directories,
            &path_to_destination_directory,
        ),
    };

    if let Err(error) = result {
        Cli::command()
            .error(
                match error {
                    Error::Custom(_) => ErrorKind::InvalidValue,
                    Error::Io(_) => ErrorKind::Io,
                    Error::DtpReader(_) => ErrorKind::Format,
                    Error::MsgReader(_) => ErrorKind::Format,
                    Error::DtpWriter(_) => ErrorKind::Io,
                },
                error,
            )
            .exit();
    }
}
