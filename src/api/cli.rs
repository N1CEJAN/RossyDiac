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
    },
    /// Converts a DTP file to a MSG file
    ConvertToMsg {
        /// The file to convert
        #[arg(short = 'f', long = "file")]
        path_to_dtp_file: String,
    },
    /// For testing: Read a DTP file
    ReadDtp,
    /// For testing: Read a MSG file
    ReadMsg,
    /// For testing: Write a DTP file
    WriteDtp {
        /// The directory to write the dtp structures to
        #[arg(short = 'd', long = "directory")]
        path_to_folder: String,
    },
    /// For testing: Write a MSG file
    WriteMsg {
        /// The directory to write the msg structures to
        #[arg(short = 'd', long = "directory")]
        path_to_folder: String,
    },
}

pub fn run() {
    let cli = Cli::parse();

    debug!("Command: {:?}", cli.command);
    let result = match cli.command {
        Command::ConvertToDtp { path_to_msg_file } => convert_to_dtp(&path_to_msg_file),
        Command::ConvertToMsg { path_to_dtp_file } => convert_to_msg(&path_to_dtp_file),
        Command::ReadDtp => read_dtp(),
        Command::ReadMsg => read_msg(),
        Command::WriteDtp { path_to_folder } => write_dtp(&path_to_folder),
        Command::WriteMsg { path_to_folder } => write_msg(&path_to_folder),
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
