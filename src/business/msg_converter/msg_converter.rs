use crate::business::error::Result;
use crate::core::{dtp, msg};

pub fn convert(structured_type: &msg::StructuredType) -> Result<dtp::DataType> {
    let name = structured_type.name().to_string();
    let mut structured_type_children = Vec::new();
    for field in structured_type.fields().iter() {
        let children = &mut convert_field(structured_type, field)?;
        structured_type_children.append(children)
    }
    let structured_type = dtp::StructuredType::new(&None, &structured_type_children);
    let data_type_kind = dtp::DataTypeKind::StructuredType(structured_type);
    Ok(dtp::DataType::new(&name, &None, &data_type_kind))
}

fn convert_field(
    structured_type: &msg::StructuredType,
    field: &msg::Field,
) -> Result<Vec<dtp::StructuredTypeChild>> {
    let mut structured_type_children = Vec::new();

    let is_helper = (1..8).any(|i| field.name().ends_with(&format!("_byte_{}", i)));
    if is_helper {
        return Ok(structured_type_children);
    }

    let var_name = convert_to_var_name(structured_type, field)?;
    let base_type = convert_to_var_base_type(structured_type, field);
    let array_size = convert_to_var_optional_array_size(structured_type, field)?;
    let initial_value = convert_to_var_optional_initial_value(structured_type, field)?;
    structured_type_children.push(dtp::StructuredTypeChild::VarDeclaration(
        dtp::VarDeclaration::new(&var_name, &base_type, &array_size, &initial_value, &None),
    ));

    if let msg::BaseType::String(Some(constraint)) | msg::BaseType::Wstring(Some(constraint)) =
        field.base_type()
    {
        let suffix = match field.base_type() {
            msg::BaseType::String(_) => "_string_bound",
            msg::BaseType::Wstring(_) => "_wstring_bound",
            _ => unreachable!("due to if let guard"),
        };
        structured_type_children.push(dtp::StructuredTypeChild::VarDeclaration(
            dtp::VarDeclaration::new(
                &format!("{}{}", var_name, suffix),
                &dtp::BaseType::ULINT,
                &None,
                &Some(dtp::InitialValue::ULINT(*constraint as u64)),
                &None,
            ),
        ));
    }

    if let Some(msg::Constraint::BoundedDynamicArray(bound)) = field.constraint() {
        structured_type_children.push(dtp::StructuredTypeChild::VarDeclaration(
            dtp::VarDeclaration::new(
                &format!("{}_array_bound", var_name),
                &dtp::BaseType::ULINT,
                &None,
                &Some(dtp::InitialValue::ULINT(*bound as u64)),
                &None,
            ),
        ));
    };

    Ok(structured_type_children)
}

fn convert_to_var_optional_initial_value(
    structured_type: &msg::StructuredType,
    field: &msg::Field,
) -> Result<Option<dtp::InitialValue>> {
    let optional_initial_value = match field.field_type() {
        msg::FieldType::Variable(optional_initial_value) => optional_initial_value.as_ref(),
        msg::FieldType::Constant(initial_value) => Some(initial_value),
    };

    if let Some(initial_value) = optional_initial_value {
        Ok(Some(convert_initial_value(
            initial_value,
            field,
            structured_type,
        )?))
    } else {
        Ok(None)
    }
}

fn get_field<'a>(
    structured_type: &'a msg::StructuredType,
    var_name: &str,
) -> Option<&'a msg::Field> {
    structured_type
        .fields()
        .iter()
        .filter(|field| field.name() == var_name)
        .next()
}

fn convert_to_var_optional_array_size(
    _: &msg::StructuredType,
    field: &msg::Field,
) -> Result<Option<dtp::ArraySize>> {
    Ok(field.constraint().map(|c| match c {
        msg::Constraint::StaticArray(capacity) => dtp::ArraySize::Static(*capacity),
        msg::Constraint::UnboundedDynamicArray | msg::Constraint::BoundedDynamicArray(_) => {
            dtp::ArraySize::Dynamic
        }
    }))
}

fn convert_to_var_base_type(_: &msg::StructuredType, field: &msg::Field) -> dtp::BaseType {
    match field.base_type() {
        msg::BaseType::Bool => dtp::BaseType::BOOL,
        msg::BaseType::Float32 => dtp::BaseType::REAL,
        msg::BaseType::Float64 => dtp::BaseType::LREAL,
        msg::BaseType::Int8 => dtp::BaseType::SINT,
        msg::BaseType::Uint8 => dtp::BaseType::USINT,
        msg::BaseType::Int16 => dtp::BaseType::INT,
        msg::BaseType::Uint16 => dtp::BaseType::UINT,
        msg::BaseType::Int32 => dtp::BaseType::DINT,
        msg::BaseType::Uint32 => dtp::BaseType::UDINT,
        msg::BaseType::Int64 => dtp::BaseType::LINT,
        msg::BaseType::Uint64 => dtp::BaseType::ULINT,
        msg::BaseType::Char => dtp::BaseType::CHAR,
        msg::BaseType::String(_) => dtp::BaseType::STRING,
        msg::BaseType::Wstring(_) => dtp::BaseType::WSTRING,
        msg::BaseType::Custom(a_ref) => dtp::BaseType::Custom(convert_reference(a_ref)),
        msg::BaseType::Byte => {
            if field.name().ends_with("_word_byte_0") {
                dtp::BaseType::WORD
            } else if field.name().ends_with("_dword_byte_0") {
                dtp::BaseType::DWORD
            } else if field.name().ends_with("_lword_byte_0") {
                dtp::BaseType::DWORD
            } else {
                dtp::BaseType::BYTE
            }
        }
    }
}

fn convert_to_var_name(_: &msg::StructuredType, field: &msg::Field) -> Result<String> {
    let mut var_name = field.name().to_string();
    if field.name().ends_with("_word_byte_0") {
        var_name = var_name
            .strip_suffix("_word_byte_0")
            .map(|a| a.to_string())
            .ok_or("unexpected_suffix_error")?;
    } else if field.name().ends_with("_dword_byte_0") {
        var_name = var_name
            .strip_suffix("_dword_byte_0")
            .map(|a| a.to_string())
            .ok_or("unexpected_suffix_error")?;
    } else if field.name().ends_with("_lword_byte_0") {
        var_name = var_name
            .strip_suffix("_lword_byte_0")
            .map(|a| a.to_string())
            .ok_or("unexpected_suffix_error")?;
    };

    match field.field_type() {
        msg::FieldType::Variable(_) => Ok(var_name),
        msg::FieldType::Constant(_) => Ok(var_name + "_CONSTANT"),
    }
}

fn convert_reference(reference: &msg::Reference) -> String {
    match reference {
        msg::Reference::Relative { file } => file.to_string(),
        msg::Reference::Absolute { package, file } => {
            format!("{}_{}", package, file)
        }
    }
}

fn convert_initial_value(
    initial_value: &msg::InitialValue,
    field: &msg::Field,
    structured_type: &msg::StructuredType,
) -> Result<dtp::InitialValue> {
    Ok(match initial_value {
        msg::InitialValue::Bool(v) => dtp::InitialValue::BOOL(*v),
        msg::InitialValue::Float32(v) => dtp::InitialValue::REAL(*v),
        msg::InitialValue::Float64(v) => dtp::InitialValue::LREAL(*v),
        msg::InitialValue::Int8(v) => dtp::InitialValue::SINT(*v),
        msg::InitialValue::Uint8(v) => dtp::InitialValue::USINT(*v),
        msg::InitialValue::Int16(v) => dtp::InitialValue::INT(*v),
        msg::InitialValue::Uint16(v) => dtp::InitialValue::UINT(*v),
        msg::InitialValue::Int32(v) => dtp::InitialValue::DINT(*v),
        msg::InitialValue::Uint32(v) => dtp::InitialValue::UDINT(*v),
        msg::InitialValue::Int64(v) => dtp::InitialValue::LINT(*v),
        msg::InitialValue::Uint64(v) => dtp::InitialValue::ULINT(*v),
        msg::InitialValue::Char(v) => dtp::InitialValue::CHAR(*v),
        msg::InitialValue::String(v) => dtp::InitialValue::STRING(v.to_string()),
        msg::InitialValue::Wstring(v) => dtp::InitialValue::WSTRING(v.to_string()),
        msg::InitialValue::Byte(v) => {
            if field.name().ends_with("_word_byte_0") {
                let concrete_word_byte_0_initial_value =
                    get_concrete_initial_value_of_byte_field(structured_type, "_word_byte_0")?;
                let concrete_word_byte_1_initial_value =
                    get_concrete_initial_value_of_byte_field(structured_type, "_word_byte_1")?;
                dtp::InitialValue::WORD(
                    ((concrete_word_byte_0_initial_value as u16) << 8)
                        | (concrete_word_byte_1_initial_value as u16),
                )
            } else if field.name().ends_with("_dword_byte_0") {
                let concrete_dword_byte_0_initial_value =
                    get_concrete_initial_value_of_byte_field(structured_type, "_dword_byte_0")?;
                let concrete_dword_byte_1_initial_value =
                    get_concrete_initial_value_of_byte_field(structured_type, "_dword_byte_1")?;
                let concrete_dword_byte_2_initial_value =
                    get_concrete_initial_value_of_byte_field(structured_type, "_dword_byte_2")?;
                let concrete_dword_byte_3_initial_value =
                    get_concrete_initial_value_of_byte_field(structured_type, "_dword_byte_3")?;
                dtp::InitialValue::DWORD(
                    ((concrete_dword_byte_0_initial_value as u32) << 24)
                        | ((concrete_dword_byte_1_initial_value as u32) << 16)
                        | ((concrete_dword_byte_2_initial_value as u32) << 8)
                        | (concrete_dword_byte_3_initial_value as u32),
                )
            } else if field.name().ends_with("_lword_byte_0") {
                let concrete_lword_byte_0_initial_value =
                    get_concrete_initial_value_of_byte_field(structured_type, "_lword_byte_0")?;
                let concrete_lword_byte_1_initial_value =
                    get_concrete_initial_value_of_byte_field(structured_type, "_lword_byte_1")?;
                let concrete_lword_byte_2_initial_value =
                    get_concrete_initial_value_of_byte_field(structured_type, "_lword_byte_2")?;
                let concrete_lword_byte_3_initial_value =
                    get_concrete_initial_value_of_byte_field(structured_type, "_lword_byte_3")?;
                let concrete_lword_byte_4_initial_value =
                    get_concrete_initial_value_of_byte_field(structured_type, "_lword_byte_4")?;
                let concrete_lword_byte_5_initial_value =
                    get_concrete_initial_value_of_byte_field(structured_type, "_lword_byte_5")?;
                let concrete_lword_byte_6_initial_value =
                    get_concrete_initial_value_of_byte_field(structured_type, "_lword_byte_6")?;
                let concrete_lword_byte_7_initial_value =
                    get_concrete_initial_value_of_byte_field(structured_type, "_lword_byte_7")?;
                dtp::InitialValue::LWORD(
                    ((concrete_lword_byte_0_initial_value as u64) << 56)
                        | ((concrete_lword_byte_1_initial_value as u64) << 48)
                        | ((concrete_lword_byte_2_initial_value as u64) << 40)
                        | ((concrete_lword_byte_3_initial_value as u64) << 32)
                        | ((concrete_lword_byte_4_initial_value as u64) << 24)
                        | ((concrete_lword_byte_5_initial_value as u64) << 16)
                        | ((concrete_lword_byte_6_initial_value as u64) << 8)
                        | (concrete_lword_byte_7_initial_value as u64),
                )
            } else {
                dtp::InitialValue::BYTE(*v)
            }
        }
        msg::InitialValue::Array(v) => dtp::InitialValue::Array(
            v.iter()
                .map(|value| convert_initial_value(value, field, structured_type))
                .collect::<Result<Vec<_>>>()?,
        ),
    })
}

fn get_concrete_initial_value_of_byte_field(
    structured_type: &msg::StructuredType,
    suffix: &str,
) -> Result<u8> {
    let byte_field = get_field(structured_type, suffix)
        .ok_or(format!("field ending with \"{}\" expected", suffix))?;
    let concrete_initial_value = match byte_field.field_type() {
        msg::FieldType::Variable(Some(msg::InitialValue::Byte(v)))
        | msg::FieldType::Constant(msg::InitialValue::Byte(v)) => Ok(*v),
        _ => Err(format!(
            "initial value for field ending with \"{}\" expected.",
            suffix
        )),
    }?;
    Ok(concrete_initial_value)
}
