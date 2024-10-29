use std::fs;

use crate::business::error::Result;
use crate::core::msg::{
    ArraySize, BaseType, BoolRepresentation, Field, FieldType, InitialValue, IntRepresentation,
    Reference, StructuredType,
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
    result.push_str(&array_size_as_string(field.array_size()));
    result.push_str(" ");
    result.push_str(field.name());
    result.push_str(&field_type_as_string(
        field.field_type(),
        field.initial_value(),
    ));
    if let Some(initial_value) = field.initial_value() {
        result.push_str(&initial_value_as_string(initial_value));
    }
    result.push_str(&comment_as_string(field.comment()));
    result
}

fn comment_as_string(comment: Option<&String>) -> String {
    comment.map_or_else(String::new, |comment| format!(" # {comment}"))
}

fn field_type_as_string(field_type: &FieldType, initial_value: Option<&InitialValue>) -> String {
    match field_type {
        FieldType::Constant => "=".to_string(),
        FieldType::Variable if initial_value.is_some() => " ".to_string(),
        FieldType::Variable => "".to_string(),
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

fn array_size_as_string(constraint: Option<&ArraySize>) -> String {
    constraint
        .map(|array_size| match array_size {
            ArraySize::Capacity(static_capacity) => {
                format!("[{}]", static_capacity)
            }
            ArraySize::Dynamic => "[]".to_string(),
            ArraySize::BoundDynamic(max_capacity) => {
                format!("[<={}]", max_capacity)
            }
        })
        .unwrap_or("".to_string())
}

fn initial_value_as_string(initial_value: &InitialValue) -> String {
    match initial_value {
        InitialValue::Bool(bool_representation) => {
            bool_representation_as_string(bool_representation)
        }
        InitialValue::Byte(int_representation)
        | InitialValue::Uint8(int_representation)
        | InitialValue::Uint16(int_representation)
        | InitialValue::Uint32(int_representation)
        | InitialValue::Uint64(int_representation)
        | InitialValue::Int8(int_representation)
        | InitialValue::Int16(int_representation)
        | InitialValue::Int32(int_representation)
        | InitialValue::Int64(int_representation) => {
            int_representation_as_string(int_representation)
        }
        InitialValue::Float32(f32) => f32.to_string(),
        InitialValue::Float64(f64) => f64.to_string(),
        InitialValue::Char(int_representation) => {
            int_representation_as_string(int_representation)
        },
        InitialValue::String(string_representation) => {
            format!("'{}'", string_representation.replace("'", "\\'"))
        }
        InitialValue::Wstring(string_representation) => {
            format!("\"{}\"", string_representation.replace("\"", "\\\""))
        }
        InitialValue::Array(v) => format!(
            "[{}]",
            v.iter()
                .map(initial_value_as_string)
                .collect::<Vec<String>>()
                .join(",")
        ),
    }
}

fn bool_representation_as_string(bool_representation: &BoolRepresentation) -> String {
    match bool_representation {
        BoolRepresentation::String(true) => "true".to_string(),
        BoolRepresentation::String(false) => "false".to_string(),
        BoolRepresentation::Binary(true) => "1".to_string(),
        BoolRepresentation::Binary(false) => "0".to_string(),
    }
}

fn int_representation_as_string(int_representation: &IntRepresentation) -> String {
    match int_representation {
        IntRepresentation::SignedDecimal(i64) => format!("{i64}"),
        IntRepresentation::UnsignedDecimal(u64) => format!("{u64}"),
        IntRepresentation::Binary(u64) => format!("0b{u64:b}"),
        IntRepresentation::Octal(u64) => format!("0o{u64:o}"),
        IntRepresentation::Hexadecimal(u64) => format!("0x{u64:X}"),
    }
}
