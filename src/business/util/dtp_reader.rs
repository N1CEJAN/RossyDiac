use log::info;
use nom::IResult;

use crate::business::error::ServiceError;
use crate::core::parser::interface::File;

pub fn read(path_to_file: &str) -> Result<File, ServiceError> {
    info!("Start reading file {:?}", path_to_file);
    let file_content = std::fs::read_to_string(path_to_file)
        .map_err(|err| ServiceError::Io(format!("{:?}", err)))?;
    let (_, parsed) = parse_file(file_content.as_str())
        .map_err(|err| ServiceError::Parser(format!("{:?}", err)))?;
    info!("Finished reading file {:?}", path_to_file);
    Ok(parsed)
}

fn parse_file(input: &str) -> IResult<&str, File> {
    todo!()
}
