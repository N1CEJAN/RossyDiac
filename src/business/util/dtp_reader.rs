use log::debug;

use crate::business::error::ServiceError;
use crate::core::parser::dtp::file_dto::FileDto;

pub struct DtpReader;

impl DtpReader {
    pub fn read(path_to_file: &str) -> Result<FileDto, ServiceError> {
        debug!("File: {:?}", path_to_file);
        let file_content = std::fs::read_to_string(path_to_file)
            .map_err(|err| ServiceError::Io(format!("{:?}", err)))?;
        debug!("File Content: {:?}", file_content);
        let dto = FileDto::new(file_content.as_str());
        Ok(dto)
    }
}
