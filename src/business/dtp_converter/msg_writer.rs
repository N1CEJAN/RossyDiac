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
            format!("[{}]", array_of_initial_values_as_string(values))
        }
    }
}

fn array_of_initial_values_as_string(values: &[InitialValue]) -> String {
    values
        .iter()
        .map(initial_value_as_string)
        .collect::<Vec<String>>()
        .join(",")
}

// fn default_initial_value(base_type: &BaseType, constraints: &[Constraint]) -> InitialValue {
//     // Annahme: Nur ein array constraint wird angegeben
//     let optional_array_constraint = constraints
//         .iter()
//         .filter(|c| match c {
//             Constraint::StaticArray(_)
//             | Constraint::UnboundedDynamicArray
//             | Constraint::BoundedDynamicArray(_) => true,
//             Constraint::BoundedString(_) => false,
//         })
//         .next();
//
//     if let Some(array_constraint) = optional_array_constraint {
//         let mut initial_values = Vec::new();
//         if let Constraint::StaticArray(capacity) = array_constraint {
//             for _ in 0..*capacity {
//                 initial_values.push(default_initial_value(base_type, &[]))
//             }
//         }
//         InitialValue::Array(initial_values)
//     } else {
//         match base_type {
//             BaseType::Bool => InitialValue::Bool(false),
//             BaseType::Byte => InitialValue::Byte(0),
//             BaseType::Float32 => InitialValue::Float32(0f32),
//             BaseType::Float64 => InitialValue::Float64(0f64),
//             BaseType::Int8 => InitialValue::Int8(0),
//             BaseType::Uint8 => InitialValue::Uint8(0),
//             BaseType::Int16 => InitialValue::Int16(0),
//             BaseType::Uint16 => InitialValue::Uint16(0),
//             BaseType::Int32 => InitialValue::Int32(0),
//             BaseType::Uint32 => InitialValue::Uint32(0),
//             BaseType::Int64 => InitialValue::Int64(0),
//             BaseType::Uint64 => InitialValue::Uint64(0),
//             // http://design.ros2.org/articles/idl_interface_definition.html
//             // A 8-bit single-byte character with a numerical value
//             // between 0 and 255 (see 7.2.6.2.1)
//             // http://design.ros2.org/articles/generated_interfaces_cpp.html#constructors
//             // Constructors: [...](note: char fields are considered numeric for C++).
//             BaseType::Char => InitialValue::Char(0),
//             BaseType::String => InitialValue::String("".to_string()),
//             BaseType::Wstring => InitialValue::Wstring("".to_string()),
//             BaseType::Custom(_) => InitialValue::Custom,
//         }
//     }
// }
