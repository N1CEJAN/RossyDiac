use std::fs;

use crate::business::error::Result;
use crate::core::msg::{
    BaseType, Constraint, EIntLiteral, Field, FieldType, InitialValue, IntLiteral, Reference,
    StructuredType,
};

pub fn write(msg_dto: &StructuredType, to_directory: &str) -> Result<()> {
    let file_name = msg_dto.name();
    let path_to_file = format!("{to_directory}{file_name}.msg");
    let file_content: String = msg_dto_as_string(msg_dto);
    fs::write(path_to_file, file_content)?;
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
    result.push_str(&base_type_as_string(field.base_type()));
    result.push_str(&constraints_as_string(field.constraint()));
    result.push_str(" ");
    result.push_str(field.name());
    result.push_str(&field_type_as_string(field.field_type()));
    result
}

fn field_type_as_string(field_type: &FieldType) -> String {
    match field_type {
        FieldType::Constant(initial_value) => {
            "=".to_string() + initial_value_as_string(initial_value).as_str()
        }
        FieldType::Variable(Some(initial_value)) => {
            " ".to_string() + initial_value_as_string(initial_value).as_str()
        }
        FieldType::Variable(None) => "".to_string(),
    }
}

fn base_type_as_string(base_type: &BaseType) -> String {
    match base_type {
        BaseType::Bool => "bool".to_string(),
        BaseType::Byte => "byte".to_string(),
        BaseType::Float32 => "float32".to_string(),
        BaseType::Float64 => "float64".to_string(),
        BaseType::Int8 => "int8".to_string(),
        BaseType::Uint8 => "uint8".to_string(),
        BaseType::Int16 => "int16".to_string(),
        BaseType::Uint16 => "uint16".to_string(),
        BaseType::Int32 => "int32".to_string(),
        BaseType::Uint32 => "uint32".to_string(),
        BaseType::Int64 => "int64".to_string(),
        BaseType::Uint64 => "uint64".to_string(),
        BaseType::Char => "char".to_string(),
        BaseType::String(constraint) => constraint
            .map(|c| format!("string<={}", c.to_string()))
            .unwrap_or_else(|| "string".to_string()),
        BaseType::Wstring(constraint) => constraint
            .map(|c| format!("wstring<={}", c.to_string()))
            .unwrap_or_else(|| "wstring".to_string()),
        BaseType::Custom(reference) => match reference {
            Reference::Relative { file } => file.clone(),
            Reference::Absolute { package, file } => format!("{}/{}", package, file),
        },
    }
}

fn constraints_as_string(constraint: Option<&Constraint>) -> String {
    constraint
        .map(|c| match c {
            Constraint::StaticArray(static_capacity) => {
                format!("[{}]", static_capacity)
            }
            Constraint::UnboundedDynamicArray => "[]".to_string(),
            Constraint::BoundedDynamicArray(max_capacity) => {
                format!("[<={}]", max_capacity)
            }
        })
        .unwrap_or("".to_string())
}

fn initial_value_as_string(initial_value: &InitialValue) -> String {
    match initial_value {
        InitialValue::Bool(value) => value.to_string(),
        InitialValue::Byte(value)
        | InitialValue::Int8(value)
        | InitialValue::Uint8(value)
        | InitialValue::Int16(value)
        | InitialValue::Uint16(value)
        | InitialValue::Int32(value)
        | InitialValue::Uint32(value)
        | InitialValue::Int64(value)
        | InitialValue::Uint64(value) => int_literal_as_string(value),
        InitialValue::Float32(value) => value.to_string(),
        InitialValue::Float64(value) => value.to_string(),
        InitialValue::Char(value) => value.to_string(),
        InitialValue::String(value) => format!("\"{}\"", value.replace("\"", "\\\"")),
        InitialValue::Wstring(value) => format!("\"{}\"", value.replace("\"", "\\\"")),
        InitialValue::Array(values) => array_of_initial_values_as_string(values),
    }
}

fn array_of_initial_values_as_string(values: &[InitialValue]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(initial_value_as_string)
            .collect::<Vec<String>>()
            .join(",")
    )
}

fn int_literal_as_string(int_literal: &IntLiteral) -> String {
    match int_literal.e_int_literal {
        EIntLiteral::DecimalInt => format!("{}", int_literal.value),
        EIntLiteral::BinaryInt => format!("0b{}", int_literal.value),
        EIntLiteral::OctalInt => format!("0o{}", int_literal.value),
        EIntLiteral::HexalInt => format!("0x{}", int_literal.value),
    }
}
