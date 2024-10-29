use crate::business::error::Result;
use crate::core::dtp::{
    ANNOTATION_NAME_ROS2_ABSOLUTE_REFERENCE, ANNOTATION_NAME_ROS2_BOUND_DYNAMIC_ARRAY,
    ANNOTATION_NAME_ROS2_CONSTANT, ANNOTATION_NAME_ROS2_DYNAMIC_ARRAY,
    ANNOTATION_NAME_ROS2_ELEMENT_COUNTER, ANNOTATION_NAME_ROS2_RELATIVE_REFERENCE,
};
use crate::core::msg::{
    ANNOTATION_NAME_IEC61499_DWORD, ANNOTATION_NAME_IEC61499_LWORD,
    ANNOTATION_NAME_IEC61499_START_INDEX, ANNOTATION_NAME_IEC61499_WORD,
};
use crate::core::{dtp, msg};

pub fn convert(module_name: &str, data_type: &dtp::DataType) -> Result<msg::StructuredType> {
    let structured_type = data_type.structured_type();
    let name = convert_data_type_name(module_name, data_type)?;
    let fields: Vec<msg::Field> = convert_structured_type(module_name, structured_type)?;
    Ok(msg::StructuredType::new(name, fields))
}

fn convert_data_type_name(module_name: &str, data_type: &dtp::DataType) -> Result<String> {
    let module_name = module_name
        .replace("_", "")
        .replace(" ", "")
        .replace("-", "");
    let full_name = data_type.name();
    Ok(full_name
        .strip_prefix(&format!("ROS2_{module_name}_msg_"))
        .unwrap_or(full_name)
        .to_string())
}

fn convert_structured_type(
    module_name: &str,
    structured_type: &dtp::StructuredType,
) -> Result<Vec<msg::Field>> {
    let mut fields: Vec<msg::Field> = Vec::new();
    for var_declaration in structured_type.var_declarations() {
        fields.append(&mut convert_var_declaration(
            module_name,
            structured_type,
            var_declaration,
        )?)
    }
    Ok(fields)
}

fn convert_var_declaration(
    module_name: &str,
    structured_type: &dtp::StructuredType,
    var_declaration: &dtp::VarDeclaration,
) -> Result<Vec<msg::Field>> {
    if is_helper(var_declaration) {
        return Ok(Vec::new());
    }

    Ok(vec![msg::Field::new(
        convert_to_field_name(var_declaration),
        convert_to_msg_base_type(module_name, var_declaration),
        convert_to_msg_array_size(var_declaration),
        convert_to_field_type(var_declaration),
        convert_to_optional_initial_value(structured_type, var_declaration)?,
        convert_to_msg_comment(var_declaration),
    )])
}

fn convert_to_msg_base_type(
    module_name: &str,
    var_declaration: &dtp::VarDeclaration,
) -> msg::BaseType {
    match var_declaration.base_type() {
        dtp::BaseType::BOOL => msg::BaseType::Bool,
        dtp::BaseType::SINT => msg::BaseType::Int8,
        dtp::BaseType::INT => msg::BaseType::Int16,
        dtp::BaseType::DINT => msg::BaseType::Int32,
        dtp::BaseType::LINT => msg::BaseType::Int64,
        dtp::BaseType::USINT => msg::BaseType::Uint8,
        dtp::BaseType::UINT => msg::BaseType::Uint16,
        dtp::BaseType::UDINT => msg::BaseType::Uint32,
        dtp::BaseType::ULINT => msg::BaseType::Uint64,
        dtp::BaseType::BYTE => msg::BaseType::Byte,
        dtp::BaseType::WORD => msg::BaseType::Uint16,
        dtp::BaseType::DWORD => msg::BaseType::Uint32,
        dtp::BaseType::LWORD => msg::BaseType::Uint64,
        dtp::BaseType::REAL => msg::BaseType::Float32,
        dtp::BaseType::LREAL => msg::BaseType::Float64,
        dtp::BaseType::CHAR => msg::BaseType::Char,
        dtp::BaseType::STRING(opt_bound) => msg::BaseType::String(opt_bound.to_owned()),
        dtp::BaseType::WSTRING(opt_bount) => msg::BaseType::Wstring(opt_bount.to_owned()),
        dtp::BaseType::Custom(value) => {
            msg::BaseType::Custom(convert_reference(module_name, var_declaration, value))
        }
    }
}

fn convert_to_msg_array_size(var_declaration: &dtp::VarDeclaration) -> Option<msg::ArraySize> {
    match var_declaration.array_size() {
        Some(dtp::ArraySize::Indexation(start, end)) => {
            Some(msg::ArraySize::Capacity((*end - *start + 1) as u64))
        }
        Some(dtp::ArraySize::Capacity(capacity)) => Some(match () {
            _ if is_dynamic_array(var_declaration) => msg::ArraySize::Dynamic,
            _ if is_bound_dynamic_array(var_declaration) => {
                msg::ArraySize::BoundDynamic(*capacity as u64)
            }
            _ => msg::ArraySize::Capacity(*capacity as u64),
        }),
        _ => None,
    }
}

fn convert_to_field_name(var_declaration: &dtp::VarDeclaration) -> String {
    var_declaration.name().to_string()
}

fn convert_to_field_type(var_declaration: &dtp::VarDeclaration) -> msg::FieldType {
    if is_constant(var_declaration) {
        msg::FieldType::Constant
    } else {
        msg::FieldType::Variable
    }
}

fn convert_to_optional_initial_value(
    structured_type: &dtp::StructuredType,
    var_declaration: &dtp::VarDeclaration,
) -> Result<Option<msg::InitialValue>> {
    var_declaration
        .initial_value()
        .map(|initial_value| convert_initial_value(structured_type, var_declaration, initial_value))
        .transpose()
}

fn convert_to_msg_comment(var_declaration: &dtp::VarDeclaration) -> Option<String> {
    let mut annotations: Vec<String> = Vec::new();
    annotations.append(&mut match *var_declaration.base_type() {
        dtp::BaseType::WORD => vec![format!("@{ANNOTATION_NAME_IEC61499_WORD}")],
        dtp::BaseType::DWORD => vec![format!("@{ANNOTATION_NAME_IEC61499_DWORD}")],
        dtp::BaseType::LWORD => vec![format!("@{ANNOTATION_NAME_IEC61499_LWORD}")],
        _ => vec![],
    });
    if let Some(dtp::ArraySize::Indexation(start, _)) = var_declaration.array_size() {
        annotations.push(format!("@{ANNOTATION_NAME_IEC61499_START_INDEX}({start})"));
    }

    let msg_comment = match annotations.is_empty() {
        false => format!("{}. ", &annotations.join(", ")),
        true => String::new(),
    } + var_declaration.comment().unwrap_or(&String::new());

    if !msg_comment.is_empty() {
        Some(msg_comment.trim().to_string())
    } else {
        None
    }
}

fn convert_initial_value(
    structured_type: &dtp::StructuredType,
    var_declaration: &dtp::VarDeclaration,
    initial_value: &dtp::InitialValue,
) -> Result<msg::InitialValue> {
    let result = match initial_value {
        dtp::InitialValue::BOOL(bool_representation) => {
            msg::InitialValue::Bool(convert_bool_representation(bool_representation))
        }
        dtp::InitialValue::BYTE(int_representation) => {
            msg::InitialValue::Byte(convert_int_representation(int_representation))
        }
        dtp::InitialValue::WORD(int_representation) => {
            msg::InitialValue::Uint16(convert_int_representation(int_representation))
        }
        dtp::InitialValue::DWORD(int_representation) => {
            msg::InitialValue::Uint32(convert_int_representation(int_representation))
        }
        dtp::InitialValue::LWORD(int_representation) => {
            msg::InitialValue::Uint64(convert_int_representation(int_representation))
        }
        dtp::InitialValue::USINT(int_representation) => {
            msg::InitialValue::Uint8(convert_int_representation(int_representation))
        }
        dtp::InitialValue::UINT(int_representation) => {
            msg::InitialValue::Uint16(convert_int_representation(int_representation))
        }
        dtp::InitialValue::UDINT(int_representation) => {
            msg::InitialValue::Uint32(convert_int_representation(int_representation))
        }
        dtp::InitialValue::ULINT(int_representation) => {
            msg::InitialValue::Uint64(convert_int_representation(int_representation))
        }
        dtp::InitialValue::SINT(int_representation) => {
            msg::InitialValue::Int8(convert_int_representation(int_representation))
        }
        dtp::InitialValue::INT(int_representation) => {
            msg::InitialValue::Int16(convert_int_representation(int_representation))
        }
        dtp::InitialValue::DINT(int_representation) => {
            msg::InitialValue::Int32(convert_int_representation(int_representation))
        }
        dtp::InitialValue::LINT(int_representation) => {
            msg::InitialValue::Int64(convert_int_representation(int_representation))
        }
        dtp::InitialValue::REAL(real_representation) => {
            msg::InitialValue::Float32(*real_representation)
        }
        dtp::InitialValue::LREAL(lreal_representation) => {
            msg::InitialValue::Float64(*lreal_representation)
        }
        dtp::InitialValue::CHAR(char_representation) => {
            msg::InitialValue::Char(convert_char_representation(char_representation))
        }
        dtp::InitialValue::STRING(string_representation) => {
            msg::InitialValue::String(convert_string_representation(string_representation))
        }
        dtp::InitialValue::WSTRING(wstring_representation) => {
            msg::InitialValue::Wstring(convert_wstring_representation(wstring_representation))
        }
        dtp::InitialValue::Array(v) => {
            let slice = if is_dynamic_array(var_declaration) {
                &v[..convert_default_dynamic_array_count(structured_type, var_declaration)?
                    as usize]
            } else {
                &v
            };
            slice
                .into_iter()
                .map(|v| convert_initial_value(structured_type, var_declaration, v))
                .collect::<Result<Vec<_>>>()
                .map(msg::InitialValue::Array)?
        }
    };
    Ok(result)
}

fn convert_reference(
    module_name: &str,
    var_declaration: &dtp::VarDeclaration,
    dtp_reference_string: &str,
) -> msg::Reference {
    let reference_parts: Vec<&str> = dtp_reference_string.split("_").collect();
    if is_absolute_reference(var_declaration) && reference_parts.len() == 4 {
        msg::Reference::Absolute {
            package: module_name.to_string(),
            file: reference_parts[3].to_string(),
        }
    } else if is_relative_reference(var_declaration) && reference_parts.len() == 4 {
        msg::Reference::Relative {
            file: reference_parts[3].to_string(),
        }
    } else {
        msg::Reference::Relative {
            file: dtp_reference_string.to_string(),
        }
    }
}

fn convert_bool_representation(
    bool_representation: &dtp::BoolRepresentation,
) -> msg::BoolRepresentation {
    match bool_representation {
        dtp::BoolRepresentation::String(bool) => msg::BoolRepresentation::String(*bool),
        dtp::BoolRepresentation::Binary(bool) => msg::BoolRepresentation::Binary(*bool),
    }
}

fn convert_int_representation(
    int_representation: &dtp::IntRepresentation,
) -> msg::IntRepresentation {
    match int_representation {
        dtp::IntRepresentation::SignedDecimal(i64) => msg::IntRepresentation::SignedDecimal(*i64),
        dtp::IntRepresentation::UnsignedDecimal(u64) => {
            msg::IntRepresentation::UnsignedDecimal(*u64)
        }
        dtp::IntRepresentation::Binary(u64) => msg::IntRepresentation::Binary(*u64),
        dtp::IntRepresentation::Octal(u64) => msg::IntRepresentation::Octal(*u64),
        dtp::IntRepresentation::Heaxdecimal(u64) => msg::IntRepresentation::Hexadecimal(*u64),
    }
}

fn convert_char_representation(
    char_representation: &dtp::CharRepresentation,
) -> msg::IntRepresentation {
    match char_representation {
        dtp::CharRepresentation::Char(char) | dtp::CharRepresentation::Hexadecimal(char) => {
            msg::IntRepresentation::Hexadecimal(*char as u64)
        }
    }
}

fn convert_string_representation(string_representation: &Vec<dtp::CharRepresentation>) -> String {
    string_representation
        .into_iter()
        .map(char_representation_to_char)
        .collect()
}

fn convert_wstring_representation(
    wstring_representation: &Vec<dtp::WcharRepresentation>,
) -> String {
    wstring_representation
        .into_iter()
        .map(wchar_representation_to_char)
        .collect()
}

fn char_representation_to_char(char_representation: &dtp::CharRepresentation) -> char {
    match char_representation {
        dtp::CharRepresentation::Char(char) | dtp::CharRepresentation::Hexadecimal(char) => *char,
    }
}

fn wchar_representation_to_char(wchar_representation: &dtp::WcharRepresentation) -> char {
    match wchar_representation {
        dtp::WcharRepresentation::Wchar(char) | dtp::WcharRepresentation::Hexadecimal(char) => {
            *char
        }
    }
}

fn is_helper(var_declaration: &dtp::VarDeclaration) -> bool {
    is_element_counter(var_declaration)
}

fn is_dynamic_array(var_declaration: &dtp::VarDeclaration) -> bool {
    filter_attributes(var_declaration, ANNOTATION_NAME_ROS2_DYNAMIC_ARRAY)
        .next()
        .is_some()
}

fn is_bound_dynamic_array(var_declaration: &dtp::VarDeclaration) -> bool {
    filter_attributes(var_declaration, ANNOTATION_NAME_ROS2_BOUND_DYNAMIC_ARRAY)
        .next()
        .is_some()
}

fn is_element_counter_of(main: &dtp::VarDeclaration, helper: &dtp::VarDeclaration) -> bool {
    filter_element_counter(helper)
        .filter(|attribute| {
            matches!(
                attribute.value(),
                dtp::InitialValue::STRING(reference)
                    if convert_string_representation(reference) == main.name()
            )
        })
        .next()
        .is_some()
}

fn is_element_counter(var_declaration: &dtp::VarDeclaration) -> bool {
    filter_element_counter(var_declaration).next().is_some()
}

fn is_relative_reference(var_declaration: &dtp::VarDeclaration) -> bool {
    filter_attributes(var_declaration, ANNOTATION_NAME_ROS2_RELATIVE_REFERENCE)
        .next()
        .is_some()
}

fn is_absolute_reference(var_declaration: &dtp::VarDeclaration) -> bool {
    filter_attributes(var_declaration, ANNOTATION_NAME_ROS2_ABSOLUTE_REFERENCE)
        .next()
        .is_some()
}

fn is_constant(var_declaration: &dtp::VarDeclaration) -> bool {
    filter_attributes(var_declaration, ANNOTATION_NAME_ROS2_CONSTANT)
        .next()
        .is_some()
}

fn filter_element_counter<'a>(
    var_declaration: &'a dtp::VarDeclaration,
) -> impl Iterator<Item = &'a dtp::Attribute> {
    filter_attributes(var_declaration, ANNOTATION_NAME_ROS2_ELEMENT_COUNTER)
}

fn filter_attributes<'a>(
    var_declaration: &'a dtp::VarDeclaration,
    attribute_name: &'static str,
) -> impl Iterator<Item = &'a dtp::Attribute> {
    var_declaration
        .attributes()
        .into_iter()
        .filter(move |attribute| attribute.name() == attribute_name)
}

fn convert_default_dynamic_array_count(
    structured_type: &dtp::StructuredType,
    var_declaration: &dtp::VarDeclaration,
) -> Result<u64> {
    structured_type
        .var_declarations()
        .into_iter()
        .find(|child| is_element_counter_of(var_declaration, child))
        .ok_or("No element counter found")?
        .initial_value()
        .map(array_bound_from_intial_value)
        .ok_or("No default element count found")?
}

fn array_bound_from_intial_value(initial_value: &dtp::InitialValue) -> Result<u64> {
    match initial_value {
        dtp::InitialValue::ULINT(dtp::IntRepresentation::SignedDecimal(i64)) if 0 < *i64 => {
            Ok(*i64 as u64)
        }
        dtp::InitialValue::ULINT(dtp::IntRepresentation::UnsignedDecimal(u64))
        | dtp::InitialValue::ULINT(dtp::IntRepresentation::Binary(u64))
        | dtp::InitialValue::ULINT(dtp::IntRepresentation::Octal(u64))
        | dtp::InitialValue::ULINT(dtp::IntRepresentation::Heaxdecimal(u64)) => Ok(*u64),
        _ => Err("A valid array bound is expected".into()),
    }
}
