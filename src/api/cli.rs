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
        /// The directory where the conversion result will be written
        #[arg(short = 'd', long = "destination-directory")]
        path_to_destination_directory: String,
        /// The name of the package the source file belongs to
        #[arg(short = 'p', long = "package-name")]
        package_name: String,
    },
    /// Converts a DTP file to a MSG file
    ConvertToMsg {
        /// The file to convert
        #[arg(short = 'f', long = "file")]
        path_to_dtp_file: String,
        /// The directory where the conversion result will be written
        #[arg(short = 'd', long = "destination-directory")]
        path_to_destination_directory: String,
        /// The name of the package the destination file will belong to
        #[arg(short = 'p', long = "package-name")]
        package_name: String,
    },
    /// Print msg file data structure
    PrintMsg {
        /// The file to read
        #[arg(short = 'f', long = "file")]
        path_to_msg_file: String,
    },
    /// Print dtp file data structure
    PrintDtp {
        /// The file to read
        #[arg(short = 'f', long = "file")]
        path_to_dtp_file: String,
    },
    /// Use in project root to apply tests
    Test,
}

pub fn run() {
    let cli = Cli::parse();

    debug!("Command: {:?}", cli.command);
    let result = match cli.command {
        Command::ConvertToDtp {
            path_to_msg_file,
            path_to_destination_directory,
            package_name,
        } => convert_to_dtp(
            &path_to_msg_file,
            &path_to_destination_directory,
            &package_name,
        ),
        Command::ConvertToMsg {
            path_to_dtp_file,
            path_to_destination_directory,
            package_name,
        } => convert_to_msg(
            &path_to_dtp_file,
            &path_to_destination_directory,
            &package_name,
        ),
        Command::PrintMsg { path_to_msg_file } => print_msg(&path_to_msg_file),
        Command::PrintDtp { path_to_dtp_file } => print_dtp(&path_to_dtp_file),
        Command::Test => {
            // hin
            let _ = convert_to_msg("test/0-dtp/Iec61499Arrayspezifikationen1.dtp", "test/1-msg/", "conversion_tests");
            let _ = convert_to_msg("test/0-dtp/Iec61499Arrayspezifikationen2.dtp", "test/1-msg/", "conversion_tests");
            let _ = convert_to_msg("test/0-dtp/Iec61499PrimitiveDatentypen.dtp", "test/1-msg/", "conversion_tests");
            let _ = convert_to_msg("test/0-dtp/Iec61499Referenzen.dtp", "test/1-msg/", "conversion_tests");
            let _ = convert_to_msg("test/0-dtp/Iec61499Standardwertliterale1.dtp", "test/1-msg/", "conversion_tests");
            let _ = convert_to_msg("test/0-dtp/Iec61499Standardwertliterale2.dtp", "test/1-msg/", "conversion_tests");
            let _ = convert_to_dtp("test/0-msg/Ros2Arrayspezifikationen1.msg", "test/1-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/0-msg/Ros2Arrayspezifikationen2.msg", "test/1-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/0-msg/Ros2Arrayspezifikationen3.msg", "test/1-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/0-msg/Ros2Arrayspezifikationen4.msg", "test/1-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/0-msg/Ros2Arrayspezifikationen5.msg", "test/1-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/0-msg/Ros2Arrayspezifikationen6.msg", "test/1-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/0-msg/Ros2Konstanten.msg", "test/1-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/0-msg/Ros2PrimitiveDatentypen.msg", "test/1-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/0-msg/Ros2Referenzen.msg", "test/1-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/0-msg/Ros2Standardwertliterale1.msg", "test/1-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/0-msg/Ros2Standardwertliterale2.msg", "test/1-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/0-msg/Ros2Standardwertliterale3.msg", "test/1-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/0-msg/Ros2Standardwertliterale4.msg", "test/1-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/0-msg/Ros2Standardwertliterale5.msg", "test/1-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/0-msg/Ros2Standardwertliterale6.msg", "test/1-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/0-msg/Ros2Standardwertliterale7.msg", "test/1-dtp/", "conversion_tests");
            
            // zurück
            let _ = convert_to_dtp("test/1-msg/Iec61499Arrayspezifikationen1.msg", "test/2-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/1-msg/Iec61499Arrayspezifikationen2.msg", "test/2-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/1-msg/Iec61499PrimitiveDatentypen.msg", "test/2-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/1-msg/Iec61499Referenzen.msg", "test/2-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/1-msg/Iec61499Standardwertliterale1.msg", "test/2-dtp/", "conversion_tests");
            let _ = convert_to_dtp("test/1-msg/Iec61499Standardwertliterale2.msg", "test/2-dtp/", "conversion_tests");
            let _ = convert_to_msg("test/1-dtp/ROS2_conversiontests_msg_Ros2Arrayspezifikationen1.dtp", "test/2-msg/", "conversion_tests");
            let _ = convert_to_msg("test/1-dtp/ROS2_conversiontests_msg_Ros2Arrayspezifikationen2.dtp", "test/2-msg/", "conversion_tests");
            let _ = convert_to_msg("test/1-dtp/ROS2_conversiontests_msg_Ros2Arrayspezifikationen3.dtp", "test/2-msg/", "conversion_tests");
            let _ = convert_to_msg("test/1-dtp/ROS2_conversiontests_msg_Ros2Arrayspezifikationen4.dtp", "test/2-msg/", "conversion_tests");
            let _ = convert_to_msg("test/1-dtp/ROS2_conversiontests_msg_Ros2Arrayspezifikationen5.dtp", "test/2-msg/", "conversion_tests");
            let _ = convert_to_msg("test/1-dtp/ROS2_conversiontests_msg_Ros2Arrayspezifikationen6.dtp", "test/2-msg/", "conversion_tests");
            let _ = convert_to_msg("test/1-dtp/ROS2_conversiontests_msg_Ros2Konstanten.dtp", "test/2-msg/", "conversion_tests");
            let _ = convert_to_msg("test/1-dtp/ROS2_conversiontests_msg_Ros2PrimitiveDatentypen.dtp", "test/2-msg/", "conversion_tests");
            let _ = convert_to_msg("test/1-dtp/ROS2_conversiontests_msg_Ros2Referenzen.dtp", "test/2-msg/", "conversion_tests");
            let _ = convert_to_msg("test/1-dtp/ROS2_conversiontests_msg_Ros2Standardwertliterale1.dtp", "test/2-msg/", "conversion_tests");
            let _ = convert_to_msg("test/1-dtp/ROS2_conversiontests_msg_Ros2Standardwertliterale2.dtp", "test/2-msg/", "conversion_tests");
            let _ = convert_to_msg("test/1-dtp/ROS2_conversiontests_msg_Ros2Standardwertliterale3.dtp", "test/2-msg/", "conversion_tests");
            let _ = convert_to_msg("test/1-dtp/ROS2_conversiontests_msg_Ros2Standardwertliterale4.dtp", "test/2-msg/", "conversion_tests");
            let _ = convert_to_msg("test/1-dtp/ROS2_conversiontests_msg_Ros2Standardwertliterale5.dtp", "test/2-msg/", "conversion_tests");
            let _ = convert_to_msg("test/1-dtp/ROS2_conversiontests_msg_Ros2Standardwertliterale6.dtp", "test/2-msg/", "conversion_tests");
            let _ = convert_to_msg("test/1-dtp/ROS2_conversiontests_msg_Ros2Standardwertliterale7.dtp", "test/2-msg/", "conversion_tests");
            Ok(())
        }
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
