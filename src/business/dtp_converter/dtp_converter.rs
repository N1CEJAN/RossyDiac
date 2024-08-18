use crate::business::error::Result;
use crate::core::{dtp, msg};

const ARRAY_BOUND_VAR_NAME_SUFFIX: &'static str = "_array_bound";
const STRING_BOUND_VAR_NAME_SUFFIX: &'static str = "_string_bound";
const WSTRING_BOUND_VAR_NAME_SUFFIX: &'static str = "_wstring_bound";
const CONSTANT_VAR_NAME_SUFFIX: &'static str = "_CONSTANT";

pub fn convert(data_type: &dtp::DataType) -> Result<msg::StructuredType> {
    let name = data_type.name().to_string();
    let fields: Vec<msg::Field> = match data_type.data_type_kind() {
        dtp::DataTypeKind::StructuredType(structured_type) => {
            convert_structured_type(structured_type)?
        }
    };
    Ok(msg::StructuredType::new(&name, fields))
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
    let is_helper = var_declaration
        .name()
        .ends_with(ARRAY_BOUND_VAR_NAME_SUFFIX)
        || var_declaration
            .name()
            .ends_with(STRING_BOUND_VAR_NAME_SUFFIX);
    if is_helper {
        return Ok(fields);
    }

    let constraint = convert_constraint(structured_type, var_declaration)?;

    // handle base_type
    match var_declaration.base_type() {
        dtp::BaseType::BOOL
        | dtp::BaseType::BYTE
        | dtp::BaseType::SINT
        | dtp::BaseType::INT
        | dtp::BaseType::DINT
        | dtp::BaseType::LINT
        | dtp::BaseType::USINT
        | dtp::BaseType::UINT
        | dtp::BaseType::UDINT
        | dtp::BaseType::ULINT
        | dtp::BaseType::REAL
        | dtp::BaseType::LREAL
        | dtp::BaseType::CHAR
        | dtp::BaseType::STRING
        | dtp::BaseType::WSTRING
        | dtp::BaseType::Custom(_) => {
            fields.push(msg::Field::new(
                &convert_base_type_directly(structured_type, var_declaration)?,
                &constraint,
                &convert_field_name(var_declaration)?,
                &convert_initial_value_directly(var_declaration)?,
            ));
        }
        dtp::BaseType::WORD | dtp::BaseType::DWORD | dtp::BaseType::LWORD => {
            let base_type = msg::BaseType::Byte;
            let bytes = convert_initial_value_word_dword_lword(var_declaration)?;
            for (index, byte) in bytes.into_iter().enumerate() {
                let field_name = convert_field_name_word_dword_lword(var_declaration, index)?;
                fields.push(msg::Field::new(&base_type, &constraint, &field_name, &byte));
            }
        }
        dtp::BaseType::TIME => todo!("time conversion"),
        dtp::BaseType::DATE => todo!("date conversion"),
        dtp::BaseType::TIME_OF_DAY | dtp::BaseType::TOD => todo!("time_of_day conversion"),
        dtp::BaseType::DATE_AND_TIME | dtp::BaseType::DT => todo!("date_and_time conversion"),
    }
    Ok(fields)
}

fn convert_base_type_directly(
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
        dtp::BaseType::REAL => msg::BaseType::Float32,
        dtp::BaseType::LREAL => msg::BaseType::Float64,
        dtp::BaseType::BYTE => msg::BaseType::Byte,
        dtp::BaseType::CHAR => msg::BaseType::Char,
        dtp::BaseType::STRING => {
            let helper_var = convert_field_name(var_declaration)? + STRING_BOUND_VAR_NAME_SUFFIX;
            let optional_bound = get_var_declaration_by_name(structured_type, &helper_var)
                .map(|var_declaration| convert_bound_var_declaration(var_declaration))
                .transpose()?;
            msg::BaseType::String(optional_bound)
        }
        dtp::BaseType::WSTRING => {
            let helper_var = convert_field_name(var_declaration)? + WSTRING_BOUND_VAR_NAME_SUFFIX;
            let optional_bound = get_var_declaration_by_name(structured_type, &helper_var)
                .map(|var_declaration| convert_bound_var_declaration(var_declaration))
                .transpose()?;
            msg::BaseType::Wstring(optional_bound)
        }
        dtp::BaseType::Custom(value) => msg::BaseType::Custom(convert_reference(value)?),
        dtp::BaseType::WORD
        | dtp::BaseType::DWORD
        | dtp::BaseType::LWORD
        | dtp::BaseType::TIME
        | dtp::BaseType::DATE
        | dtp::BaseType::TIME_OF_DAY
        | dtp::BaseType::TOD
        | dtp::BaseType::DATE_AND_TIME
        | dtp::BaseType::DT => return Err("Direct BaseType conversion not possible".into()),
    };
    Ok(result)
}

fn convert_constraint(
    structured_type: &dtp::StructuredType,
    var_declaration: &dtp::VarDeclaration,
) -> Result<Option<msg::Constraint>> {
    let constraint = match var_declaration.array_size() {
        Some(dtp::ArraySize::Dynamic) => {
            let optional_constraint_var_declaration = get_var_declaration_by_name(
                structured_type,
                &(convert_field_name(var_declaration)? + ARRAY_BOUND_VAR_NAME_SUFFIX),
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
            Some(msg::Constraint::StaticArray(end - start + 1))
        }
        None => None,
    };
    Ok(constraint)
}

fn convert_field_name(var_declaration: &dtp::VarDeclaration) -> Result<String> {
    let mut name = var_declaration.name();
    if var_declaration.name().ends_with(CONSTANT_VAR_NAME_SUFFIX) {
        name = name
            .strip_suffix(CONSTANT_VAR_NAME_SUFFIX)
            .ok_or(format!("_CONSTANT suffix not found on name \"{}\"", name))?;
    }
    Ok(name.to_string())
}

fn convert_field_name_word_dword_lword(
    var_declaration: &dtp::VarDeclaration,
    index: usize,
) -> Result<String> {
    let var_name = convert_field_name(var_declaration)?;
    Ok(match var_declaration.base_type() {
        dtp::BaseType::WORD => format!("{}_word_byte_{}", var_name, index),
        dtp::BaseType::DWORD => format!("{}_dword_byte_{}", var_name, index),
        dtp::BaseType::LWORD => format!("{}_lword_byte_{}", var_name, index),
        _ => return Err(format!("no byte string name conversion found for {}", var_name).into()),
    })
}

fn convert_initial_value_directly(var_declaration: &dtp::VarDeclaration) -> Result<msg::FieldType> {
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
        dtp::InitialValue::INT(v) => msg::InitialValue::Int16(*v),
        dtp::InitialValue::DINT(v) => msg::InitialValue::Int32(*v),
        dtp::InitialValue::LINT(v) => msg::InitialValue::Int64(*v),
        dtp::InitialValue::USINT(v) => msg::InitialValue::Uint8(*v),
        dtp::InitialValue::UINT(v) => msg::InitialValue::Uint16(*v),
        dtp::InitialValue::UDINT(v) => msg::InitialValue::Uint32(*v),
        dtp::InitialValue::ULINT(v) => msg::InitialValue::Uint64(*v),
        dtp::InitialValue::REAL(v) => msg::InitialValue::Float32(*v),
        dtp::InitialValue::LREAL(v) => msg::InitialValue::Float64(*v),
        dtp::InitialValue::BYTE(v) => msg::InitialValue::Byte(*v),
        dtp::InitialValue::CHAR(v) => msg::InitialValue::Char(*v),
        dtp::InitialValue::STRING(v) => msg::InitialValue::String(v.clone()),
        dtp::InitialValue::WSTRING(v) => msg::InitialValue::Wstring(v.clone()),
        dtp::InitialValue::Array(v) => v
            .into_iter()
            .map(convert_initial_value_directly2)
            .collect::<Result<Vec<_>>>()
            .map(msg::InitialValue::Array)?,
        _ => return Err(format!("No direct conversion found for {:?}", initial_value).into()),
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

fn convert_initial_value_word_dword_lword(
    var_declaration: &dtp::VarDeclaration,
) -> Result<Vec<msg::FieldType>> {
    let optional_initial_values = var_declaration
        .initial_value()
        .as_ref()
        .map(convert_initial_value_word_dword_lword2)
        .transpose()?;

    if let Some(initial_values) = optional_initial_values {
        initial_values
            .into_iter()
            .map(|initial_value| convert_field_type(var_declaration, Some(initial_value)))
            .collect::<Result<Vec<_>>>()
    } else {
        let byte_count = match var_declaration.base_type() {
            dtp::BaseType::WORD => 2,
            dtp::BaseType::DWORD => 4,
            dtp::BaseType::LWORD => 8,
            _ => unreachable!("due to check in initial value conversion"),
        };
        Ok((0..byte_count)
            .map(|_| msg::FieldType::Variable(None))
            .collect())
    }
}

fn convert_initial_value_word_dword_lword2(
    initial_value: &dtp::InitialValue,
) -> Result<Vec<msg::InitialValue>> {
    if let dtp::InitialValue::Array(array) = initial_value {
        let result = array
            .iter()
            .map(convert_initial_value_word_dword_lword2)
            .collect::<Result<Vec<Vec<msg::InitialValue>>>>()?;
        let len = result[0].len();
        let mut iters: Vec<_> = result.into_iter().map(|n| n.into_iter()).collect();
        Ok((0..len)
            .map(|_| {
                msg::InitialValue::Array(
                    iters
                        .iter_mut()
                        .map(|n| n.next().unwrap())
                        .collect::<Vec<msg::InitialValue>>(),
                )
            })
            .collect::<Vec<msg::InitialValue>>())
    } else {
        let (value, byte_count) = match initial_value {
            dtp::InitialValue::WORD(v) => (*v as u64, 2),
            dtp::InitialValue::DWORD(v) => (*v as u64, 4),
            dtp::InitialValue::LWORD(v) => (*v, 8),
            _ => return Err(format!("No byte string conversion for {initial_value:?}").into()),
        };

        Ok((0..byte_count)
            .rev()
            .map(|i| msg::InitialValue::Byte((value >> (i * 8)) as u8))
            .collect())
    }
}

fn convert_field_type(
    var_declaration: &dtp::VarDeclaration,
    optional_initial_value: Option<msg::InitialValue>,
) -> Result<msg::FieldType> {
    let is_constant = var_declaration.name().ends_with(CONSTANT_VAR_NAME_SUFFIX);
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
    if let Some(dtp::InitialValue::ULINT(initial_value)) = bound_var_declaration.initial_value() {
        Ok(*initial_value as usize)
    } else {
        Err(format!(
            "Invalid InitialValue of helper VarDeclaration {}",
            bound_var_declaration.name()
        )
        .into())
    }
}

fn convert_reference(dtp_reference_string: &str) -> Result<msg::Reference> {
    let reference_parts: Vec<&str> = dtp_reference_string.split("__").collect();
    if reference_parts.len() == 3 {
        Ok(msg::Reference::Absolute {
            package: reference_parts[0].to_string(),
            file: reference_parts[2].to_string(),
        })
    } else if reference_parts.len() > 1 {
        Ok(msg::Reference::Relative {
            file: dtp_reference_string.to_string(),
        })
    } else {
        Err(format!("Empty file reference \"{}\"", dtp_reference_string).into())
    }
}
