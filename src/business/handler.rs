use crate::business::error::ServiceError;

pub fn convert_to_dtp(path_to_file: String) -> Result<(), ServiceError> {
    let file_content = std::fs::read_to_string(path_to_file)
        .map_err(|error| ServiceError::Io(error))?;
    println!("Content:\n\n{}", file_content);
    Ok(())
}

pub fn convert_to_msg(path_to_file: String) -> Result<(), ServiceError> {
    let file_content = std::fs::read_to_string(path_to_file)
        .map_err(|error| ServiceError::Io(error))?;
    println!("Content: {}", file_content);
    Ok(())
}
