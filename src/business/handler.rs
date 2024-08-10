use crate::business::dtp_converter::*;
use crate::business::error::Result;
use crate::business::msg_converter::*;

pub fn convert_to_dtp(
    path_to_source_file: &str,
    path_to_source_directories: &Vec<String>,
    path_to_destination_directory: &str,
) -> Result<()> {
    let msg_dtos = msg_reader::read(path_to_source_file, path_to_source_directories)?;
    let dtp_dtos = msg_converter::convert(msg_dtos)?;
    dtp_writer::write(dtp_dtos, path_to_destination_directory)?;
    Ok(())
}

pub fn convert_to_msg(
    path_to_source_file: &str,
    path_to_source_directories: &Vec<String>,
    path_to_destination_directory: &str,
) -> Result<()> {
    let dtp_dtos = dtp_reader::read(path_to_source_file, path_to_source_directories)?;
    let msg_dtos = dtp_converter::convert(dtp_dtos)?;
    msg_writer::write(msg_dtos, path_to_destination_directory)?;
    Ok(())
}
