use crate::core::parser::msg::field_name_dto::FieldNameDto;
use crate::core::parser::msg::field_type::FieldType;

#[derive(Debug, Clone)]
pub struct FieldDto {
    field_type: FieldType,
    field_name: FieldNameDto,
}

impl FieldDto {
    pub fn new(field_type: FieldType, field_name: FieldNameDto) -> Self {
        Self {
            field_type,
            field_name,
        }
    }
}
