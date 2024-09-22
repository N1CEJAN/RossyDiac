use crate::business::error::Result;
use crate::core::{dtp, msg};

pub fn convert(package_name: &str, data_type: &dtp::DataType) -> Result<msg::StructuredType> {
    let name = convert_data_type_name(package_name, data_type)?;
    let fields: Vec<msg::Field> = match data_type.data_type_kind() {
        dtp::DataTypeKind::StructuredType(structured_type) => {
            convert_structured_type(package_name, structured_type)?
        }
    };
    Ok(msg::StructuredType::new(&name, fields))
}

fn convert_data_type_name(package_name: &str, data_type: &dtp::DataType) -> Result<String> {
    let package_name = package_name
        .replace("_", "")
        .replace(" ", "")
        .replace("-", "");
    let full_name = data_type.name();
    Ok(full_name
        .strip_prefix(&format!("ROS2_{package_name}_msg_"))
        .unwrap_or(full_name)
        .to_string())
}

fn convert_structured_type(module_name: &str, structured_type: &dtp::StructuredType) -> Result<Vec<msg::Field>> {
    let mut fields: Vec<msg::Field> = Vec::new();

    for structured_type_child in structured_type.children() {
        match structured_type_child {
            dtp::StructuredTypeChild::VarDeclaration(var_declaration) => fields.append(
                &mut convert_var_declaration(module_name, structured_type, var_declaration)?,
            ),
        }
    }
    Ok(fields)
}

fn convert_var_declaration(
    module_name: &str,
    structured_type: &dtp::StructuredType,
    var_declaration: &dtp::VarDeclaration,
) -> Result<Vec<msg::Field>> {
    let is_helper = var_declaration
        .attributes()
        .into_iter()
        .find(|attr| attr.name == "ROS2_ElementCounter");
    if is_helper.is_some() {
        return Ok(Vec::new());
    }

    Ok(vec![msg::Field::new(
        &convert_to_msg_base_type(module_name, var_declaration)?,
        &convert_to_msg_constraint(var_declaration)?,
        &convert_to_field_name(var_declaration),
        &convert_to_msg_initial_value(structured_type, var_declaration)?,
        &convert_to_msg_comment(var_declaration),
    )])
}

fn convert_to_msg_comment(var_declaration: &dtp::VarDeclaration) -> Option<String> {
    let mut annotations: Vec<String> = Vec::new();
    annotations.append(&mut match *var_declaration.base_type() {
        dtp::BaseType::WORD => vec!["@IEC61499_WORD".to_string()],
        dtp::BaseType::DWORD => vec!["@IEC61499_DWORD".to_string()],
        dtp::BaseType::LWORD => vec!["@IEC61499_LWORD".to_string()],
        _ => vec![],
    });
    if let Some(dtp::ArraySize::Static(dtp::Capacity::Shifted(start, _))) =
        var_declaration.array_size()
    {
        annotations.push(format!("@IEC61499_StartIndex({start})"));
    }

    let msg_comment = match annotations.is_empty() {
        false => format!("{}. ", &annotations.join(", ")),
        true => String::new(),
    } + var_declaration.comment().as_ref().unwrap_or(&String::new());

    if !msg_comment.is_empty() {
        Some(msg_comment.trim().to_string())
    } else {
        None
    }
}

fn convert_to_msg_base_type(module_name: &str, var_declaration: &dtp::VarDeclaration) -> Result<msg::BaseType> {
    let result = match var_declaration.base_type() {
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
        dtp::BaseType::STRING(opt_bound) => msg::BaseType::String(opt_bound.clone()),
        dtp::BaseType::WSTRING(opt_bount) => msg::BaseType::Wstring(opt_bount.clone()),
        dtp::BaseType::Custom(value) => {
            msg::BaseType::Custom(convert_reference(module_name, var_declaration, value)?)
        }
    };
    Ok(result)
}

fn convert_to_msg_constraint(
    var_declaration: &dtp::VarDeclaration,
) -> Result<Option<msg::Constraint>> {
    match var_declaration.array_size() {
        Some(dtp::ArraySize::Dynamic) => Err("Dynamic arrays are not supported".into()),
        Some(dtp::ArraySize::Static(dtp::Capacity::Shifted(start, end))) => Ok(Some(
            msg::Constraint::StaticArray((end - start + 1) as usize),
        )),
        Some(dtp::ArraySize::Static(dtp::Capacity::InPlace(_)))
            if is_dynamic_array(var_declaration) =>
        {
            Ok(Some(msg::Constraint::UnboundedDynamicArray))
        }
        Some(dtp::ArraySize::Static(dtp::Capacity::InPlace(_)))
            if is_bound_dynamic_array(var_declaration) =>
        {
            let array_bound = convert_array_bound(var_declaration)
                .ok_or("Unsigned integer required for \"ROS2_BoundDynamicArray\"")?;
            Ok(Some(msg::Constraint::BoundedDynamicArray(array_bound)))
        }
        Some(dtp::ArraySize::Static(dtp::Capacity::InPlace(capacity))) => {
            Ok(Some(msg::Constraint::StaticArray(*capacity)))
        }
        _ => Ok(None),
    }
}

fn convert_to_field_name(var_declaration: &dtp::VarDeclaration) -> String {
    var_declaration.name().to_string()
}

fn convert_to_msg_initial_value(
    structured_type: &dtp::StructuredType,
    var_declaration: &dtp::VarDeclaration,
) -> Result<msg::FieldType> {
    let optional_initial_value = var_declaration
        .initial_value()
        .as_ref()
        .map(|initial_value| {
            convert_initial_value_directly2(structured_type, var_declaration, initial_value)
        })
        .transpose()?;
    convert_field_type(var_declaration, optional_initial_value)
}

fn convert_initial_value_directly2(
    structured_type: &dtp::StructuredType,
    var_declaration: &dtp::VarDeclaration,
    initial_value: &dtp::InitialValue,
) -> Result<msg::InitialValue> {
    let result = match initial_value {
        dtp::InitialValue::BOOL(v) => msg::InitialValue::Bool(convert_bool_literal(v)),
        dtp::InitialValue::SINT(v) => msg::InitialValue::Int8(convert_int_literal(v)),
        dtp::InitialValue::INT(v) => msg::InitialValue::Int16(convert_int_literal(v)),
        dtp::InitialValue::DINT(v) => msg::InitialValue::Int32(convert_int_literal(v)),
        dtp::InitialValue::LINT(v) => msg::InitialValue::Int64(convert_int_literal(v)),
        dtp::InitialValue::USINT(v) => msg::InitialValue::Uint8(convert_int_literal(v)),
        dtp::InitialValue::UINT(v) => msg::InitialValue::Uint16(convert_int_literal(v)),
        dtp::InitialValue::UDINT(v) => msg::InitialValue::Uint32(convert_int_literal(v)),
        dtp::InitialValue::ULINT(v) => msg::InitialValue::Uint64(convert_int_literal(v)),
        dtp::InitialValue::BYTE(v) => msg::InitialValue::Byte(convert_int_literal(v)),
        dtp::InitialValue::WORD(v) => msg::InitialValue::Uint16(convert_int_literal(v)),
        dtp::InitialValue::DWORD(v) => msg::InitialValue::Uint32(convert_int_literal(v)),
        dtp::InitialValue::LWORD(v) => msg::InitialValue::Uint64(convert_int_literal(v)),
        dtp::InitialValue::REAL(v) => msg::InitialValue::Float32(*v),
        dtp::InitialValue::LREAL(v) => msg::InitialValue::Float64(*v),
        dtp::InitialValue::CHAR(v) => msg::InitialValue::Char(convert_char_literal(v)),
        dtp::InitialValue::STRING(v) => msg::InitialValue::String(v.clone()),
        dtp::InitialValue::WSTRING(v) => msg::InitialValue::Wstring(v.clone()),
        dtp::InitialValue::Array(v) => {
            let slice = if is_dynamic_array(var_declaration) {
                &v[..convert_default_dynamic_array_count(structured_type, var_declaration)?]
            } else {
                &v
            };
            slice
                .into_iter()
                .map(|v| convert_initial_value_directly2(structured_type, var_declaration, v))
                .collect::<Result<Vec<_>>>()
                .map(msg::InitialValue::Array)?
        }
    };
    Ok(result)
}

fn convert_char_literal(dtp_char_literal: &dtp::CharLiteral) -> msg::IntLiteral {
    match dtp_char_literal {
        dtp::CharLiteral::Value(char) | dtp::CharLiteral::Hex(char) => {
            msg::IntLiteral::HexalInt(*char as u64)
        }
    }
}

fn convert_bool_literal(dtp_bool_literal: &dtp::BoolLiteral) -> msg::BoolLiteral {
    match dtp_bool_literal {
        dtp::BoolLiteral::String(bool) => msg::BoolLiteral::String(*bool),
        dtp::BoolLiteral::Int(bool) => msg::BoolLiteral::Int(*bool),
    }
}

fn convert_int_literal(dtp_int_literal: &dtp::IntLiteral) -> msg::IntLiteral {
    match dtp_int_literal {
        dtp::IntLiteral::SignedDecimalInt(i64) => msg::IntLiteral::SignedDecimalInt(*i64),
        dtp::IntLiteral::UnsignedDecimalInt(u64) => msg::IntLiteral::UnsignedDecimalInt(*u64),
        dtp::IntLiteral::BinaryInt(u64) => msg::IntLiteral::BinaryInt(*u64),
        dtp::IntLiteral::OctalInt(u64) => msg::IntLiteral::OctalInt(*u64),
        dtp::IntLiteral::HexalInt(u64) => msg::IntLiteral::HexalInt(*u64),
    }
}

fn convert_field_type(
    var_declaration: &dtp::VarDeclaration,
    optional_initial_value: Option<msg::InitialValue>,
) -> Result<msg::FieldType> {
    if is_constant(var_declaration) {
        optional_initial_value
            .map(msg::FieldType::Constant)
            .ok_or("Invalid constant found".into())
    } else {
        Ok(msg::FieldType::Variable(optional_initial_value))
    }
}

fn convert_reference(
    module_name: &str,
    var_declaration: &dtp::VarDeclaration,
    dtp_reference_string: &str,
) -> Result<msg::Reference> {
    let reference_parts: Vec<&str> = dtp_reference_string.split("_").collect();
    if is_absolute_reference(var_declaration) && reference_parts.len() == 4 {
        Ok(msg::Reference::Absolute {
            package: module_name.to_string(),
            file: reference_parts[3].to_string(),
        })
    } else if is_relative_reference(var_declaration) && reference_parts.len() == 4 {
        Ok(msg::Reference::Relative {
            file: reference_parts[3].to_string(),
        })
    } else if reference_parts.len() == 1 {
        Ok(msg::Reference::Relative {
            file: dtp_reference_string.to_string(),
        })
    } else {
        Err("Invalid reference found".into())
    }
}

fn convert_default_dynamic_array_count(
    structured_type: &dtp::StructuredType,
    var_declaration: &dtp::VarDeclaration,
) -> Result<usize> {
    structured_type
        .children()
        .iter()
        .find_map(|child| match child {
            dtp::StructuredTypeChild::VarDeclaration(helper)
                if is_element_counter_of(var_declaration, helper) =>
            {
                Some(helper)
            }
            _ => None,
        })
        .map(|helper| {
            helper
                .initial_value()
                .as_ref()
                .map(extract_usize_from_intial_value)
                .flatten()
                .ok_or("Invalid element counter found")
        })
        .transpose()?
        .ok_or("No element counter found".into())
}

fn convert_array_bound(var_declaration: &dtp::VarDeclaration) -> Option<usize> {
    var_declaration
        .attributes()
        .into_iter()
        .find(|attribute| attribute.name == "ROS2_BoundDynamicArray")
        .map(|initial_value| &initial_value.value)
        .and_then(extract_usize_from_intial_value)
}

fn extract_usize_from_intial_value(initial_value: &dtp::InitialValue) -> Option<usize> {
    match initial_value {
        dtp::InitialValue::ULINT(dtp::IntLiteral::UnsignedDecimalInt(u64))
        | dtp::InitialValue::ULINT(dtp::IntLiteral::BinaryInt(u64))
        | dtp::InitialValue::ULINT(dtp::IntLiteral::OctalInt(u64))
        | dtp::InitialValue::ULINT(dtp::IntLiteral::HexalInt(u64)) => Some(*u64 as usize),
        _ => None,
    }
}

fn is_element_counter_of(main: &dtp::VarDeclaration, helper: &dtp::VarDeclaration) -> bool {
    helper.attributes().into_iter().any(|attribute| {
        let has_annotation = attribute.name == "ROS2_ElementCounter";
        let annotation_references_main = matches!(
            &attribute.value,
            dtp::InitialValue::STRING(reference) if reference == main.name()
        );
        has_annotation && annotation_references_main
    })
}

fn is_dynamic_array(var_declaration: &dtp::VarDeclaration) -> bool {
    var_declaration
        .attributes()
        .into_iter()
        .any(|attr| attr.name == "ROS2_DynamicArray")
}

fn is_bound_dynamic_array(var_declaration: &dtp::VarDeclaration) -> bool {
    var_declaration
        .attributes()
        .into_iter()
        .any(|attr| attr.name == "ROS2_BoundDynamicArray")
}

fn is_relative_reference(var_declaration: &dtp::VarDeclaration) -> bool {
    var_declaration
        .attributes()
        .into_iter()
        .any(|attr| attr.name == "ROS2_RelativeReference")
}

fn is_absolute_reference(var_declaration: &dtp::VarDeclaration) -> bool {
    var_declaration
        .attributes()
        .into_iter()
        .any(|attr| attr.name == "ROS2_AbsoluteReference")
}

fn is_constant(var_declaration: &dtp::VarDeclaration) -> bool {
    var_declaration
        .attributes()
        .into_iter()
        .any(|attr| attr.name == "ROS2_Constant")
}
