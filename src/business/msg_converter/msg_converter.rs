use crate::business::error::Result;
use crate::core::{dtp, msg};

const WORD_SUFFIX: &'static str = "_word";
const DWORD_SUFFIX: &'static str = "_dword";
const LWORD_SUFFIX: &'static str = "_lword";
const STRING_BOUND_SUFFIX: &'static str = "_string_bound";
const ARRAY_BOUND_SUFFIX: &'static str = "_array_bound";
const CONSTANT_SUFFIX: &'static str = "_CONSTANT";

pub fn convert(package_name: &str, structured_type: &msg::StructuredType) -> Result<dtp::DataType> {
    let name = convert_structured_type_name(package_name, structured_type.name());
    let mut structured_type_children = Vec::new();
    for field in structured_type.fields().iter() {
        let children = &mut convert_field(package_name, field)?;
        structured_type_children.append(children)
    }
    let structured_type = dtp::StructuredType::new(&None, &structured_type_children);
    let data_type_kind = dtp::DataTypeKind::StructuredType(structured_type);
    Ok(dtp::DataType::new(&name, &None, &data_type_kind))
}

fn convert_structured_type_name(package_name: &str, structured_type_name: &str) -> String {
    let package_name = package_name
        .replace("_", "")
        .replace(" ", "")
        .replace("-", "");
    format!("ROS2_{package_name}_msg_{structured_type_name}")
}

fn convert_field(package_name: &str, field: &msg::Field) -> Result<Vec<dtp::StructuredTypeChild>> {
    let mut structured_type_children = Vec::new();

    let var_name = convert_to_var_name(field)?;
    let base_type = convert_to_var_base_type(package_name, field);
    let array_size = convert_to_var_optional_array_size(field)?;
    let initial_value = convert_to_var_optional_initial_value(field)?;
    structured_type_children.push(dtp::StructuredTypeChild::VarDeclaration(
        dtp::VarDeclaration::new(&var_name, &base_type, &array_size, &initial_value, &None),
    ));

    if let msg::BaseType::String(Some(constraint)) | msg::BaseType::Wstring(Some(constraint)) =
        field.base_type()
    {
        structured_type_children.push(dtp::StructuredTypeChild::VarDeclaration(
            dtp::VarDeclaration::new(
                &format!("{var_name}{STRING_BOUND_SUFFIX}"),
                &dtp::BaseType::ULINT,
                &None,
                &Some(dtp::InitialValue::ULINT(dtp::IntLiteral {
                    value: constraint.to_string(),
                    int_type: None,
                    e_int_literal: dtp::EIntLiteral::DecimalInt,
                })),
                &None,
            ),
        ));
    }

    if let Some(msg::Constraint::BoundedDynamicArray(bound)) = field.constraint() {
        structured_type_children.push(dtp::StructuredTypeChild::VarDeclaration(
            dtp::VarDeclaration::new(
                &format!("{var_name}{ARRAY_BOUND_SUFFIX}"),
                &dtp::BaseType::ULINT,
                &None,
                &Some(dtp::InitialValue::ULINT(dtp::IntLiteral {
                    value: bound.to_string(),
                    int_type: None,
                    e_int_literal: dtp::EIntLiteral::DecimalInt,
                })),
                &None,
            ),
        ));
    };

    Ok(structured_type_children)
}

fn convert_to_var_name(field: &msg::Field) -> Result<String> {
    let mut var_name = field.name();
    if field.name().ends_with(WORD_SUFFIX) {
        var_name = var_name
            .strip_suffix(WORD_SUFFIX)
            .ok_or("unexpected_suffix_error")?;
    } else if field.name().ends_with(DWORD_SUFFIX) {
        var_name = var_name
            .strip_suffix(DWORD_SUFFIX)
            .ok_or("unexpected_suffix_error")?;
    } else if field.name().ends_with(LWORD_SUFFIX) {
        var_name = var_name
            .strip_suffix(LWORD_SUFFIX)
            .ok_or("unexpected_suffix_error")?;
    };
    let var_name = var_name.to_string();

    Ok(match field.field_type() {
        msg::FieldType::Variable(_) => var_name,
        msg::FieldType::Constant(_) => var_name + CONSTANT_SUFFIX,
    })
}

fn convert_to_var_base_type(package_name: &str, field: &msg::Field) -> dtp::BaseType {
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
        msg::BaseType::Byte if field.name().ends_with(WORD_SUFFIX) => dtp::BaseType::WORD,
        msg::BaseType::Byte if field.name().ends_with(DWORD_SUFFIX) => dtp::BaseType::DWORD,
        msg::BaseType::Byte if field.name().ends_with(LWORD_SUFFIX) => dtp::BaseType::LWORD,
        msg::BaseType::Byte => dtp::BaseType::BYTE,
        msg::BaseType::Char => dtp::BaseType::CHAR,
        msg::BaseType::String(_) => dtp::BaseType::STRING,
        msg::BaseType::Wstring(_) => dtp::BaseType::WSTRING,
        msg::BaseType::Custom(a_ref) => {
            dtp::BaseType::Custom(convert_reference(package_name, a_ref))
        }
    }
}

fn convert_to_var_optional_array_size(field: &msg::Field) -> Result<Option<dtp::ArraySize>> {
    Ok(field.constraint().map(|c| match c {
        msg::Constraint::StaticArray(capacity) => {
            // Erklärung: ROS2 kann nicht anders indexieren,
            // weswegen derzeit die Information einer anderen
            // Indexierung verloren geht.
            // cross(InPlace(c)) => InPlace(c)
            // cross(Shifted(s, e)) => InPlace(e-s+1)
            dtp::ArraySize::Static(dtp::Capacity::InPlace(*capacity))
        }
        msg::Constraint::UnboundedDynamicArray | msg::Constraint::BoundedDynamicArray(_) => {
            dtp::ArraySize::Dynamic
        }
    }))
}

fn convert_to_var_optional_initial_value(field: &msg::Field) -> Result<Option<dtp::InitialValue>> {
    let optional_initial_value = match field.field_type() {
        msg::FieldType::Variable(optional_initial_value) => optional_initial_value.as_ref(),
        msg::FieldType::Constant(initial_value) => Some(initial_value),
    };

    match optional_initial_value {
        Some(initial_value) => Ok(Some(convert_initial_value(initial_value, field)?)),
        None => Ok(None),
    }
}

fn convert_reference(package_name: &str, reference: &msg::Reference) -> String {
    match reference {
        msg::Reference::Relative { file } => convert_structured_type_name(package_name, file),
        msg::Reference::Absolute { package, file } => convert_structured_type_name(package, file),
    }
}

fn convert_initial_value(
    initial_value: &msg::InitialValue,
    field: &msg::Field,
) -> Result<dtp::InitialValue> {
    Ok(match initial_value {
        msg::InitialValue::Bool(v) => dtp::InitialValue::BOOL(*v),
        msg::InitialValue::Float32(v) => dtp::InitialValue::REAL(*v),
        msg::InitialValue::Float64(v) => dtp::InitialValue::LREAL(*v),
        msg::InitialValue::Int8(v) => dtp::InitialValue::SINT(convert_int_literal(v)),
        msg::InitialValue::Uint8(v) => dtp::InitialValue::USINT(convert_int_literal(v)),
        msg::InitialValue::Int16(v) => dtp::InitialValue::INT(convert_int_literal(v)),
        msg::InitialValue::Uint16(v) => dtp::InitialValue::UINT(convert_int_literal(v)),
        msg::InitialValue::Int32(v) => dtp::InitialValue::DINT(convert_int_literal(v)),
        msg::InitialValue::Uint32(v) => dtp::InitialValue::UDINT(convert_int_literal(v)),
        msg::InitialValue::Int64(v) => dtp::InitialValue::LINT(convert_int_literal(v)),
        msg::InitialValue::Uint64(v) => dtp::InitialValue::ULINT(convert_int_literal(v)),
        msg::InitialValue::Byte(v) if field.name().ends_with(WORD_SUFFIX) => {
            dtp::InitialValue::WORD(convert_int_literal(v))
        }
        msg::InitialValue::Byte(v) if field.name().ends_with(DWORD_SUFFIX) => {
            dtp::InitialValue::DWORD(convert_int_literal(v))
        }
        msg::InitialValue::Byte(v) if field.name().ends_with(LWORD_SUFFIX) => {
            dtp::InitialValue::LWORD(convert_int_literal(v))
        }
        msg::InitialValue::Byte(v) => dtp::InitialValue::BYTE(convert_int_literal(v)),
        msg::InitialValue::Char(v) => dtp::InitialValue::CHAR(*v),
        msg::InitialValue::String(v) => dtp::InitialValue::STRING(v.to_string()),
        msg::InitialValue::Wstring(v) => dtp::InitialValue::WSTRING(v.to_string()),
        msg::InitialValue::Array(v) => dtp::InitialValue::Array(
            v.iter()
                .map(|value| convert_initial_value(value, field))
                .collect::<Result<Vec<_>>>()?,
        ),
    })
}

fn convert_int_literal(int_literal: &msg::IntLiteral) -> dtp::IntLiteral {
    let e_int_literal = match int_literal.e_int_literal {
        msg::EIntLiteral::DecimalInt => dtp::EIntLiteral::DecimalInt,
        msg::EIntLiteral::BinaryInt => dtp::EIntLiteral::BinaryInt,
        msg::EIntLiteral::OctalInt => dtp::EIntLiteral::OctalInt,
        msg::EIntLiteral::HexalInt => dtp::EIntLiteral::HexalInt,
    };
    dtp::IntLiteral {
        int_type: None,
        value: int_literal.value.clone(),
        e_int_literal,
    }
}
