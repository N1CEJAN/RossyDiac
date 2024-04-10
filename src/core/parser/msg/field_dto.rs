use crate::core::parser::msg::field_name_dto::FieldNameDto;
use crate::core::parser::msg::field_type::FieldType;

#[derive(Debug, Clone)]
pub struct FieldDto {
    field_type: FieldType,
    field_name: FieldNameDto,
    field_default_value: Option<String>,
}

impl FieldDto {
    pub fn new(
        field_type: FieldType,
        field_name: FieldNameDto,
        field_default_value: Option<String>,
    ) -> Self {
        Self {
            field_type,
            field_name,
            field_default_value,
        }
    }
}
