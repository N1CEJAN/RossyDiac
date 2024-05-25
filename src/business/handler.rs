use log::debug;

use crate::business::dtp_converter::{dtp_converter, dtp_reader, msg_writer};
use crate::business::error::Result;
use crate::business::msg_converter::{dtp_writer, msg_converter, msg_reader};

pub fn convert_to_dtp(path_to_file: &str) -> Result<()> {
    let structured_type = msg_reader::read(path_to_file)?;
    debug!("MSG file: {:#?}", structured_type);
    let custom_data_types = msg_converter::convert(structured_type)?;
    debug!("DTP files: {:#?}", custom_data_types);
    dtp_writer::write(custom_data_types)?;
    Ok(())
}

pub fn convert_to_msg(path_to_file: &str) -> Result<()> {
    let custom_data_type = dtp_reader::read(path_to_file)?;
    debug!("DTP file: {:#?}", custom_data_type);
    let structured_types = dtp_converter::convert(custom_data_type)?;
    debug!("MSG file: {:#?}", structured_types);
    msg_writer::write(structured_types)?;
    Ok(())
}
