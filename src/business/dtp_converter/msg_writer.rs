use std::fs;

use crate::business::error::Result;
use crate::core::msg::{BaseType, Constraint, Field, FieldType, InitialValue, StructuredType};

pub fn write(msg_dtos: Vec<StructuredType>, to_directory: &str) -> Result<()> {
    for msg_dto in msg_dtos.iter() {
        let file_name = msg_dto.name();
        let path_to_file = format!("{to_directory}{file_name}.msg");
        let file_content: String = msg_dto_as_string(msg_dto);
        fs::write(path_to_file, file_content)?
    }
    Ok(())
}

fn msg_dto_as_string(msg_dto: &StructuredType) -> String {
    let mut result: String = String::new();
    for field in msg_dto.fields().iter() {
        result.push_str(&field_as_string(field));
        result.push_str("\r\n");
    }
    result
}

fn field_as_string(field: &Field) -> String {
    let mut result: String = String::new();
    result.push_str(base_type_as_string(field.base_type()));
    result.push_str(&constraints_as_string(field.constraints()));
    result.push_str(" ");
    result.push_str(field.name());
    result.push_str(&field_type_as_string(field.field_type()));
    result.push_str(&initial_value_as_string(field.initial_value()));
    result
}

fn field_type_as_string(field_type: &FieldType) -> String {
    match field_type {
        FieldType::Variable => " ".to_string(),
        FieldType::Constant => "=".to_string(),
    }
}

fn base_type_as_string(base_type: &BaseType) -> &str {
    match base_type {
        BaseType::Bool => "bool",
        BaseType::Byte => "byte",
        BaseType::Float32 => "float32",
        BaseType::Float64 => "float64",
        BaseType::Int8 => "int8",
        BaseType::Uint8 => "uint8",
        BaseType::Int16 => "int16",
        BaseType::Uint16 => "uint16",
        BaseType::Int32 => "int32",
        BaseType::Uint32 => "uint32",
        BaseType::Int64 => "int64",
        BaseType::Uint64 => "uint64",
        BaseType::Char => "char",
        BaseType::String => "string",
        BaseType::Wstring => "wstring",
        BaseType::Custom(custom) => custom,
    }
}

fn constraints_as_string(constraints: &Vec<Constraint>) -> String {
    let mut result = String::new();
    let mut optional_string_constraint = None;
    let mut optional_array_constriant = None;

    // Annahme: Maximal 2 constraints und davon ist nur einer einer ein array constraint
    for constraint in constraints {
        match constraint {
            Constraint::BoundedString(upper_bound) => {
                optional_string_constraint = Some(format!("<={}", upper_bound));
            }
            Constraint::BoundedDynamicArray(max_capacity) => {
                optional_array_constriant = Some(format!("[<={}]", max_capacity));
            }
            Constraint::StaticArray(static_capacity) => {
                optional_array_constriant = Some(format!("[{}]", static_capacity));
            }
            Constraint::UnboundedDynamicArray => {
                optional_array_constriant = Some("[]".to_string());
            }
        }
    }

    if let Some(string_constraint) = optional_string_constraint {
        result += &string_constraint;
    }
    if let Some(array_constraint) = optional_array_constriant {
        result += &array_constraint;
    }
    result
}

fn initial_value_as_string(initial_value: &InitialValue) -> String {
    match initial_value {
        InitialValue::Bool(value) => value.to_string(),
        InitialValue::Byte(value) => value.to_string(),
        InitialValue::Float32(value) => value.to_string(),
        InitialValue::Float64(value) => value.to_string(),
        InitialValue::Int8(value) => value.to_string(),
        InitialValue::Uint8(value) => value.to_string(),
        InitialValue::Int16(value) => value.to_string(),
        InitialValue::Uint16(value) => value.to_string(),
        InitialValue::Int32(value) => value.to_string(),
        InitialValue::Uint32(value) => value.to_string(),
        InitialValue::Int64(value) => value.to_string(),
        InitialValue::Uint64(value) => value.to_string(),
        InitialValue::Char(value) => value.to_string(),
        InitialValue::String(value) => format!("\"{}\"", value.replace("\"", "\\\"")),
        InitialValue::Wstring(value) => format!("\"{}\"", value.replace("\"", "\\\"")),
        InitialValue::Custom => "".to_string(),
        InitialValue::Array(values) => {
            format!(
                "[{}]",
                values
                    .iter()
                    .map(initial_value_as_string)
                    .collect::<Vec<String>>()
                    .join(",")
            )
        }
    }
}
