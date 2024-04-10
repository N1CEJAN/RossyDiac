use crate::core::parser::msg::field_dto::FieldDto;

#[derive(Debug)]
pub struct FileDto {
    fields: Vec<FieldDto>,
}

impl FileDto {
    pub fn new(fields: Vec<FieldDto>) -> Self {
        Self { fields }
    }
}
