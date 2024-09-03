use log::debug;
use crate::business::error::{Error, Result};
use crate::core::{dtp, msg};

const WORD_SUFFIX: &'static str = "_word";
const DWORD_SUFFIX: &'static str = "_dword";
const LWORD_SUFFIX: &'static str = "_lword";
const STRING_BOUND_SUFFIX: &'static str = "_string_bound";
const ARRAY_BOUND_SUFFIX: &'static str = "_array_bound";
const CONSTANT_SUFFIX: &'static str = "_CONSTANT";

pub fn convert(package_name: &str, data_type: &dtp::DataType) -> Result<msg::StructuredType> {
    let name = convert_data_type_name(package_name, data_type)?;
    let fields: Vec<msg::Field> = match data_type.data_type_kind() {
        dtp::DataTypeKind::StructuredType(structured_type) => {
            convert_structured_type(structured_type)?
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
        .unwrap_or_else(|| full_name)
        .to_string())
}

fn convert_structured_type(structured_type: &dtp::StructuredType) -> Result<Vec<msg::Field>> {
    let mut fields: Vec<msg::Field> = Vec::new();

    for structured_type_child in structured_type.children() {
        match structured_type_child {
            dtp::StructuredTypeChild::VarDeclaration(var_declaration) => fields.append(
                &mut convert_var_declaration(structured_type, var_declaration)?,
            ),
        }
    }
    Ok(fields)
}

fn convert_var_declaration(
    structured_type: &dtp::StructuredType,
    var_declaration: &dtp::VarDeclaration,
) -> Result<Vec<msg::Field>> {
    let mut fields: Vec<msg::Field> = Vec::new();

    // skip helper
    let is_helper = var_declaration.name().ends_with(ARRAY_BOUND_SUFFIX)
        || var_declaration.name().ends_with(STRING_BOUND_SUFFIX);
    if is_helper {
        return Ok(fields);
    }

    match var_declaration.base_type() {
        dtp::BaseType::TIME => todo!("time conversion"),
        dtp::BaseType::DATE => todo!("date conversion"),
        dtp::BaseType::TIME_OF_DAY | dtp::BaseType::TOD => todo!("time_of_day conversion"),
        dtp::BaseType::DATE_AND_TIME | dtp::BaseType::DT => todo!("date_and_time conversion"),
        _ => {
            fields.push(msg::Field::new(
                &convert_to_msg_base_type(structured_type, var_declaration)?,
                &convert_to_msg_constraint(structured_type, var_declaration)?,
                &convert_to_field_name(var_declaration)?,
                &convert_to_msg_initial_value(var_declaration)?,
            ));
        }
    }
    Ok(fields)
}

fn convert_to_msg_base_type(
    structured_type: &dtp::StructuredType,
    var_declaration: &dtp::VarDeclaration,
) -> Result<msg::BaseType> {
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
        dtp::BaseType::STRING => {
            msg::BaseType::String(extract_string_bound(structured_type, var_declaration)?)
        }
        dtp::BaseType::WSTRING => {
            msg::BaseType::Wstring(extract_string_bound(structured_type, var_declaration)?)
        }
        dtp::BaseType::Custom(value) => msg::BaseType::Custom(convert_reference(value)?),
        _ => return Err("Direct BaseType conversion not possible".into()),
    };
    Ok(result)
}

fn extract_string_bound(
    structured_type: &dtp::StructuredType,
    var_declaration: &dtp::VarDeclaration,
) -> Result<Option<usize>> {
    let helper_var = convert_to_field_name(var_declaration)? + STRING_BOUND_SUFFIX;
    get_var_declaration_by_name(structured_type, &helper_var)
        .map(|var_declaration| convert_bound_var_declaration(var_declaration))
        .transpose()
}

fn convert_to_msg_constraint(
    structured_type: &dtp::StructuredType,
    var_declaration: &dtp::VarDeclaration,
) -> Result<Option<msg::Constraint>> {
    let constraint = match var_declaration.array_size() {
        Some(dtp::ArraySize::Dynamic) => {
            let optional_constraint_var_declaration = get_var_declaration_by_name(
                structured_type,
                &(convert_to_field_name(var_declaration)? + ARRAY_BOUND_SUFFIX),
            );
            if let Some(constraint_var_declaration) = optional_constraint_var_declaration {
                let bound = convert_bound_var_declaration(constraint_var_declaration)?;
                Some(msg::Constraint::BoundedDynamicArray(bound))
            } else {
                Some(msg::Constraint::UnboundedDynamicArray)
            }
        }
        Some(dtp::ArraySize::Static(dtp::Capacity::InPlace(capacity))) => {
            Some(msg::Constraint::StaticArray(*capacity))
        }
        // Annahme: Es werden nu 0..x Shifts angegeben
        Some(dtp::ArraySize::Static(dtp::Capacity::Shifted(start, end))) => {
            Some(msg::Constraint::StaticArray((end - start + 1) as usize))
        }
        None => None,
    };
    Ok(constraint)
}

fn convert_to_field_name(var_declaration: &dtp::VarDeclaration) -> Result<String> {
    let mut name = var_declaration.name();
    if var_declaration.name().ends_with(CONSTANT_SUFFIX) {
        name = name
            .strip_suffix(CONSTANT_SUFFIX)
            .ok_or(format!("_CONSTANT suffix not found on name \"{name}\""))?;
    }
    Ok(match var_declaration.base_type() {
        dtp::BaseType::WORD => format!("{}{}", name, WORD_SUFFIX),
        dtp::BaseType::DWORD => format!("{}{}", name, DWORD_SUFFIX),
        dtp::BaseType::LWORD => format!("{}{}", name, LWORD_SUFFIX),
        _ => name.to_string(),
    })
}

fn convert_to_msg_initial_value(var_declaration: &dtp::VarDeclaration) -> Result<msg::FieldType> {
    let optional_initial_value = var_declaration
        .initial_value()
        .as_ref()
        .map(|initial_value| convert_initial_value_directly2(initial_value))
        .transpose()?;
    convert_field_type(var_declaration, optional_initial_value)
}

fn convert_initial_value_directly2(initial_value: &dtp::InitialValue) -> Result<msg::InitialValue> {
    let result = match initial_value {
        dtp::InitialValue::BOOL(v) => msg::InitialValue::Bool(*v),
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
        dtp::InitialValue::CHAR(v) => msg::InitialValue::Char(*v),
        dtp::InitialValue::STRING(v) => msg::InitialValue::String(v.clone()),
        dtp::InitialValue::WSTRING(v) => msg::InitialValue::Wstring(v.clone()),
        dtp::InitialValue::Array(v) => v
            .into_iter()
            .map(convert_initial_value_directly2)
            .collect::<Result<Vec<_>>>()
            .map(msg::InitialValue::Array)?,
        _ => return Err(format!("No direct conversion found for {initial_value:?}").into()),
    };
    Ok(result)
}

fn convert_int_literal(initial_value: &dtp::IntLiteral) -> msg::IntLiteral {
    let e_int_literal = match &initial_value.e_int_literal {
        dtp::EIntLiteral::DecimalInt => msg::EIntLiteral::DecimalInt,
        dtp::EIntLiteral::BinaryInt => msg::EIntLiteral::BinaryInt,
        dtp::EIntLiteral::OctalInt => msg::EIntLiteral::OctalInt,
        dtp::EIntLiteral::HexalInt => msg::EIntLiteral::HexalInt,
    };
    msg::IntLiteral {
        // Entscheidung: Die Underscores erhalte ich nicht,
        // weil es verschwendet Konvertierungsaufwand ist,
        // nur für ein sauber cross-transpiling.
        value: initial_value.value.replace("_", ""),
        e_int_literal,
    }
}

fn convert_field_type(
    var_declaration: &dtp::VarDeclaration,
    optional_initial_value: Option<msg::InitialValue>,
) -> Result<msg::FieldType> {
    let is_constant = var_declaration.name().ends_with(CONSTANT_SUFFIX);
    if is_constant {
        optional_initial_value.map(msg::FieldType::Constant).ok_or(
            format!(
                "No valid value found for constant \"{}\"",
                var_declaration.name()
            )
            .into(),
        )
    } else {
        Ok(msg::FieldType::Variable(optional_initial_value))
    }
}

fn get_var_declaration_by_name<'a>(
    dtp_structured_type: &'a dtp::StructuredType,
    name: &str,
) -> Option<&'a dtp::VarDeclaration> {
    dtp_structured_type
        .children()
        .iter()
        .find_map(|child| match child {
            dtp::StructuredTypeChild::VarDeclaration(var_declaration) => {
                (var_declaration.name() == name).then_some(var_declaration)
            }
        })
}

fn convert_bound_var_declaration(bound_var_declaration: &dtp::VarDeclaration) -> Result<usize> {
    if let Some(dtp::InitialValue::ULINT(int_literal)) = bound_var_declaration.initial_value() {
        Ok(int_literal.value.parse().map_err(Error::custom)?)
    } else {
        Err(format!(
            "Invalid InitialValue of helper VarDeclaration {}",
            bound_var_declaration.name()
        )
        .into())
    }
}

fn convert_reference(dtp_reference_string: &str) -> Result<msg::Reference> {
    let reference_parts: Vec<&str> = dtp_reference_string.split("_").collect();
    debug!("Reference parts {:?}", reference_parts);
    if reference_parts.len() == 4 {
        Ok(msg::Reference::Absolute {
            package: reference_parts[1].to_string(),
            file: reference_parts[3].to_string(),
        })
    } else if reference_parts.len() == 1 {
        Ok(msg::Reference::Relative {
            file: dtp_reference_string.to_string(),
        })
    } else {
        Err(format!("Empty file reference \"{dtp_reference_string}\"").into())
    }
}
