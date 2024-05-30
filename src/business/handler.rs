use log::debug;

use crate::business::dtp_converter::*;
use crate::business::error::Result;
use crate::business::msg_converter::*;

pub fn convert_to_dtp(path_to_file: &str) -> Result<()> {
    todo!()
}

pub fn convert_to_msg(path_to_file: &str) -> Result<()> {
    todo!()
}

pub fn read_dtp() -> Result<()> {
    let dtp_dto = dtp_reader::read("test/resources/Test.dtp")?;
    debug!("DTP file: {:#?}", dtp_dto);
    Ok(())
}

pub fn read_msg() -> Result<()> {
    let msg_dto = msg_reader::read("test/resources/Test.msg")?;
    debug!("MSG file: {:#?}", msg_dto);
    Ok(())
}

pub fn write_dtp(to_directory: &str) -> Result<()> {
    let dtp_dto = dtp_reader::read("test/resources/Test.dtp")?;
    debug!("DTP file: {:#?}", dtp_dto);
    dtp_writer::write(vec![dtp_dto], to_directory)?;
    Ok(())
}

pub fn write_msg(to_directory: &str) -> Result<()> {
    let msg_dto = msg_reader::read("test/resources/Test.msg")?;
    debug!("MSG file: {:#?}", msg_dto);
    msg_writer::write(vec![msg_dto], to_directory)?;
    Ok(())
}
