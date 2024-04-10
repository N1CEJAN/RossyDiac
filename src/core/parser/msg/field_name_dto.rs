#[derive(Debug, Clone)]
pub struct FieldNameDto {
    field_name: String,
}

impl FieldNameDto {
    pub fn new(field_name: &str) -> Self {
        Self {
            field_name: field_name.to_string().clone(),
        }
    }
}
