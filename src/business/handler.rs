use log::debug;
use crate::business::dtp_converter::*;
use crate::business::error::Result;
use crate::business::msg_converter::*;

pub fn convert_to_dtp(
    path_to_source_file: &str,
    path_to_destination_directory: &str,
) -> Result<()> {
    let msg_dto = msg_reader::read(path_to_source_file)?;
    debug!("msg_dto: {:?}", msg_dto);
    let dtp_dto = msg_converter::convert(&msg_dto)?;
    debug!("dtp_dto: {:?}", dtp_dto);
    dtp_writer::write(dtp_dto, path_to_destination_directory)?;
    Ok(())
}

pub fn convert_to_msg(
    path_to_source_file: &str,
    path_to_destination_directory: &str,
) -> Result<()> {
    let dtp_dto = dtp_reader::read(path_to_source_file)?;
    debug!("dtp_dto: {:?}", dtp_dto);
    let msg_dto = dtp_converter::convert(&dtp_dto)?;
    debug!("msg_dto: {:?}", msg_dto);
    msg_writer::write(&msg_dto, path_to_destination_directory)?;
    Ok(())
}
