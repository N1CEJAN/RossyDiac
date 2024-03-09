use crate::business::error::ServiceError;
use crate::core::dtp::DtpFileDto;

pub struct DtpReader;

impl DtpReader {
    pub fn read(path_to_file: &str) -> Result<DtpFileDto, ServiceError> {
        let file_content = std::fs::read_to_string(path_to_file)
            .map_err(|error| ServiceError::Io(error))?;
        let dto = DtpFileDto::new(file_content.as_str());
        Ok(dto)
    }
}