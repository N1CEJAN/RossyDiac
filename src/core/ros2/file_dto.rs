use crate::core::ros2::field_dto::FieldDto;

#[derive(Debug)]
pub struct FileDto {
    fields: Vec<FieldDto>,
}

impl FileDto {
    pub fn new(fields: Vec<FieldDto>) -> Self {
        Self { fields }
    }
}
