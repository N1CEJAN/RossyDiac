use log::debug;

use crate::business::error::ServiceError;
use crate::business::util::dtp_reader::DtpReader;
use crate::business::util::msg_reader::read;

pub fn convert_to_dtp(path_to_file: &str) -> Result<(), ServiceError> {
    let msg_file_dto = read(path_to_file);
    debug!("DTO: {:?}", msg_file_dto);
    Ok(())
}

pub fn convert_to_msg(path_to_file: &str) -> Result<(), ServiceError> {
    let dtp_file_dto = DtpReader::read(path_to_file);
    debug!("DTO: {:?}", dtp_file_dto);
    Ok(())
}
