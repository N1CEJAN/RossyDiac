use log::debug;
use crate::business::error::Result;

use crate::business::util::{dtp_reader, msg_reader};

pub fn convert_to_dtp(path_to_file: &str) -> Result<()> {
    let msg_file_dto = msg_reader::read(path_to_file)?;
    debug!("DTP file: {:#?}", msg_file_dto);
    Ok(())
}

pub fn convert_to_msg(path_to_file: &str) -> Result<()> {
    let dtp_file_dto = dtp_reader::read(path_to_file)?;
    debug!("MSG file: {:#?}", dtp_file_dto);
    Ok(())
}
