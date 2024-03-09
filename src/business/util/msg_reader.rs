use crate::business::error::ServiceError;
use crate::core::msg::MsgFileDto;

pub struct MsgReader;

impl MsgReader {
    pub fn read(path_to_file: &str) -> Result<MsgFileDto, ServiceError> {
        println!("File: {:?}", path_to_file);
        let file_content = std::fs::read_to_string(path_to_file)
            .map_err(|error| ServiceError::Io(error))?;
        let dto = MsgFileDto::new(file_content.as_str());
        Ok(dto)
    }
}