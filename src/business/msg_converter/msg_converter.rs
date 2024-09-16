use crate::business::error::Result;
use crate::core::{dtp, msg};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, opt, recognize};
use nom::sequence::{delimited, tuple};
use nom::{Finish, IResult};

const ELEMENT_COUNTER_SUFFIX: &'static str = "_element_counter";

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
    let comment = convert_to_var_comment(field)?;
    let attributes = convert_to_attributes(field)?;
    structured_type_children.push(dtp::StructuredTypeChild::VarDeclaration(
        dtp::VarDeclaration::new(
            &var_name,
            &base_type,
            &array_size,
            &initial_value,
            &comment,
            &attributes,
        ),
    ));

    if let Some(msg::Constraint::BoundedDynamicArray(_))
    | Some(msg::Constraint::UnboundedDynamicArray) = field.constraint()
    {
        let default_count = compute_element_counter_default_count(field);
        structured_type_children.push(dtp::StructuredTypeChild::VarDeclaration(
            dtp::VarDeclaration::new(
                &format!("{var_name}{ELEMENT_COUNTER_SUFFIX}"),
                &dtp::BaseType::ULINT,
                &None,
                &Some(dtp::InitialValue::ULINT(
                    dtp::IntLiteral::UnsignedDecimalInt(default_count),
                )),
                &None,
                &vec![dtp::Attribute {
                    name: "ROS2_ElementCounter".to_string(),
                    base_type: dtp::BaseType::STRING,
                    value: dtp::InitialValue::STRING(field.name().to_string()),
                    comment: None,
                }],
            ),
        ));
    };

    Ok(structured_type_children)
}

fn convert_to_var_comment(field: &msg::Field) -> Result<Option<String>> {
    if let Some(comment) = field.comment() {
        if comment.is_empty() {
            return Ok(None);
        }
        let comment = if comment.starts_with('@') {
            comment
                .find('.')
                .map(|end_pos| &comment[end_pos + 1..])
                .unwrap_or(comment)
        } else {
            comment
        }
        .trim()
        .to_string();

        if comment.is_empty() {
            Ok(None)
        } else {
            Ok(Some(comment))
        }
    } else {
        Ok(None)
    }
}

fn compute_element_counter_default_count(field: &msg::Field) -> u64 {
    match field.field_type() {
        msg::FieldType::Variable(Some(msg::InitialValue::Array(initial_value)))
        | msg::FieldType::Constant(msg::InitialValue::Array(initial_value)) => {
            initial_value.len() as u64
        }
        _ => 0,
    }
}

fn convert_to_attributes(field: &msg::Field) -> Result<Vec<dtp::Attribute>> {
    let mut attributes = Vec::new();
    if let Some(msg::Constraint::UnboundedDynamicArray) = field.constraint() {
        attributes.push(dtp::Attribute {
            name: "ROS2_DynamicArray".to_string(),
            base_type: dtp::BaseType::BOOL,
            value: dtp::InitialValue::BOOL(dtp::BoolLiteral::Int(true)),
            comment: None,
        })
    }
    if let Some(msg::Constraint::BoundedDynamicArray(bound)) = field.constraint() {
        attributes.push(dtp::Attribute {
            name: "ROS2_BoundDynamicArray".to_string(),
            base_type: dtp::BaseType::ULINT,
            value: dtp::InitialValue::ULINT(dtp::IntLiteral::UnsignedDecimalInt(*bound as u64)),
            comment: None,
        })
    }
    if let msg::BaseType::String(Some(bound)) | msg::BaseType::Wstring(Some(bound)) =
        field.base_type()
    {
        attributes.push(dtp::Attribute {
            name: "ROS2_BoundString".to_string(),
            base_type: dtp::BaseType::ULINT,
            value: dtp::InitialValue::ULINT(dtp::IntLiteral::UnsignedDecimalInt(*bound as u64)),
            comment: None,
        })
    }
    if let msg::FieldType::Constant(_) = field.field_type() {
        attributes.push(dtp::Attribute {
            name: "ROS2_CONSTANT".to_string(),
            base_type: dtp::BaseType::BOOL,
            value: dtp::InitialValue::BOOL(dtp::BoolLiteral::Int(true)),
            comment: None,
        })
    }
    if let msg::BaseType::Custom(msg::Reference::Relative { .. }) = field.base_type() {
        attributes.push(dtp::Attribute {
            name: "ROS2_RelativeReference".to_string(),
            base_type: dtp::BaseType::BOOL,
            value: dtp::InitialValue::BOOL(dtp::BoolLiteral::Int(true)),
            comment: None,
        })
    }
    if let msg::BaseType::Custom(msg::Reference::Absolute { .. }) = field.base_type() {
        attributes.push(dtp::Attribute {
            name: "ROS2_AbsoluteReference".to_string(),
            base_type: dtp::BaseType::BOOL,
            value: dtp::InitialValue::BOOL(dtp::BoolLiteral::Int(true)),
            comment: None,
        })
    }
    Ok(attributes)
}

fn convert_to_var_name(field: &msg::Field) -> Result<String> {
    Ok(field.name().to_string())
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
        msg::BaseType::Byte if is_word(field) => dtp::BaseType::WORD,
        msg::BaseType::Byte if is_dword(field) => dtp::BaseType::DWORD,
        msg::BaseType::Byte if is_lword(field) => dtp::BaseType::LWORD,
        msg::BaseType::Byte => dtp::BaseType::BYTE,
        msg::BaseType::Char => dtp::BaseType::CHAR,
        msg::BaseType::String(_) => dtp::BaseType::STRING,
        msg::BaseType::Wstring(_) => dtp::BaseType::WSTRING,
        msg::BaseType::Custom(a_ref) => {
            dtp::BaseType::Custom(convert_reference(package_name, a_ref))
        }
    }
}

fn is_word(field: &msg::Field) -> bool {
    field
        .comment()
        .is_some_and(|comment| comment.contains("@IEC61499_WORD"))
}

fn is_dword(field: &msg::Field) -> bool {
    field
        .comment()
        .is_some_and(|comment| comment.contains("@IEC61499_DWORD"))
}

fn is_lword(field: &msg::Field) -> bool {
    field
        .comment()
        .is_some_and(|comment| comment.contains("@IEC61499_LWORD"))
}

fn is_shifted_static_array(field: &msg::Field) -> bool {
    field
        .comment()
        .is_some_and(|comment| comment.contains("@IEC61499_StartIndex"))
}

fn get_start_index(field: &msg::Field) -> Result<i64> {
    field
        .comment()
        .and_then(|comment| {
            comment
                .find("@IEC61499_StartIndex(")
                .map(|pos| &comment[pos..])
        })
        .map_or(Ok(0), |input| {
            Ok(parse_start_index(input)
                .map_err(|err| err.to_owned())
                .finish()?
                .1)
        })
}

fn parse_start_index(input: &str) -> IResult<&str, i64> {
    map_res(
        delimited(
            tag("@IEC61499_StartIndex("),
            recognize(tuple((opt(alt((tag("-"), tag("+")))), digit1))),
            tag(")")
        ),
        |str: &str| i64::from_str_radix(str, 10),
    )(input)
}

fn convert_to_var_optional_array_size(field: &msg::Field) -> Result<Option<dtp::ArraySize>> {
    Ok(match field.constraint() {
        Some(msg::Constraint::StaticArray(capacity)) if is_shifted_static_array(field) => {
            let start = get_start_index(field)?;
            let end = start + (*capacity as i64) - 1;
            Some(dtp::ArraySize::Static(dtp::Capacity::Shifted(start, end)))
        }
        Some(msg::Constraint::StaticArray(capacity)) => {
            Some(dtp::ArraySize::Static(dtp::Capacity::InPlace(*capacity)))
        }
        Some(msg::Constraint::UnboundedDynamicArray) => {
            Some(dtp::ArraySize::Static(dtp::Capacity::InPlace(100)))
        }
        Some(msg::Constraint::BoundedDynamicArray(bound)) => {
            Some(dtp::ArraySize::Static(dtp::Capacity::InPlace(*bound)))
        }
        None => None,
    })
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
        msg::InitialValue::Bool(v) => dtp::InitialValue::BOOL(convert_bool_literal(v)),
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
        msg::InitialValue::Byte(v) if is_word(field) => {
            dtp::InitialValue::WORD(convert_int_literal(v))
        }
        msg::InitialValue::Byte(v) if is_dword(field) => {
            dtp::InitialValue::DWORD(convert_int_literal(v))
        }
        msg::InitialValue::Byte(v) if is_lword(field) => {
            dtp::InitialValue::LWORD(convert_int_literal(v))
        }
        msg::InitialValue::Byte(v) => dtp::InitialValue::BYTE(convert_int_literal(v)),
        msg::InitialValue::Char(v) => dtp::InitialValue::CHAR(convert_to_char_literal(v)?),
        msg::InitialValue::String(v) => dtp::InitialValue::STRING(v.to_string()),
        msg::InitialValue::Wstring(v) => dtp::InitialValue::WSTRING(v.to_string()),
        msg::InitialValue::Array(v) => {
            // Determine new capacity based on field constraint
            let new_capacity = match field.constraint() {
                Some(msg::Constraint::BoundedDynamicArray(capacity)) => *capacity,
                Some(msg::Constraint::UnboundedDynamicArray) => 100,
                _ => v.len(),
            };

            // Convert initial values
            let mut vec = v
                .iter()
                .map(|value| convert_initial_value(value, field))
                .collect::<Result<Vec<_>>>()?;

            // Add filler values as needed
            if new_capacity > vec.len() {
                let sample_initial_value = v.iter().next();
                vec.extend(vec![
                    create_filler_initial_value(field, sample_initial_value);
                    new_capacity - vec.len()
                ]);
            }

            dtp::InitialValue::Array(vec)
        }
    })
}

fn create_filler_initial_value(
    field: &msg::Field,
    sample_initial_value: Option<&msg::InitialValue>,
) -> dtp::InitialValue {
    sample_initial_value.map_or_else(
        || create_default_initial_value(field),
        |sample_initial_value| match sample_initial_value {
            msg::InitialValue::Bool(literal) => {
                dtp::InitialValue::BOOL(create_filler_bool_literal(Some(literal)))
            }
            msg::InitialValue::Byte(literal) => {
                dtp::InitialValue::BYTE(create_filler_int_literal(Some(literal)))
            }
            msg::InitialValue::Uint16(literal) if is_word(field) => {
                dtp::InitialValue::WORD(create_filler_int_literal(Some(literal)))
            }
            msg::InitialValue::Uint32(literal) if is_dword(field) => {
                dtp::InitialValue::DWORD(create_filler_int_literal(Some(literal)))
            }
            msg::InitialValue::Uint64(literal) if is_lword(field) => {
                dtp::InitialValue::LWORD(create_filler_int_literal(Some(literal)))
            }
            msg::InitialValue::Uint8(literal) => {
                dtp::InitialValue::USINT(create_filler_int_literal(Some(literal)))
            }
            msg::InitialValue::Uint16(literal) => {
                dtp::InitialValue::UINT(create_filler_int_literal(Some(literal)))
            }
            msg::InitialValue::Uint32(literal) => {
                dtp::InitialValue::UDINT(create_filler_int_literal(Some(literal)))
            }
            msg::InitialValue::Uint64(literal) => {
                dtp::InitialValue::ULINT(create_filler_int_literal(Some(literal)))
            }
            msg::InitialValue::Int8(literal) => {
                dtp::InitialValue::SINT(create_filler_int_literal(Some(literal)))
            }
            msg::InitialValue::Int16(literal) => {
                dtp::InitialValue::INT(create_filler_int_literal(Some(literal)))
            }
            msg::InitialValue::Int32(literal) => {
                dtp::InitialValue::DINT(create_filler_int_literal(Some(literal)))
            }
            msg::InitialValue::Int64(literal) => {
                dtp::InitialValue::LINT(create_filler_int_literal(Some(literal)))
            }
            msg::InitialValue::Float32(_) => dtp::InitialValue::REAL(0f32),
            msg::InitialValue::Float64(_) => dtp::InitialValue::LREAL(0f64),
            msg::InitialValue::Char(_) => dtp::InitialValue::CHAR(create_filler_char_literal()),
            msg::InitialValue::String(_) => dtp::InitialValue::STRING(String::new()),
            msg::InitialValue::Wstring(_) => dtp::InitialValue::WSTRING(String::new()),
            msg::InitialValue::Array(_) => unimplemented!(),
        },
    )
}

fn create_default_initial_value(field: &msg::Field) -> dtp::InitialValue {
    match field.base_type() {
        msg::BaseType::Bool => dtp::InitialValue::BOOL(create_filler_bool_literal(None)),
        msg::BaseType::Byte => dtp::InitialValue::BYTE(create_filler_int_literal(None)),
        msg::BaseType::Uint16 if is_word(field) => {
            dtp::InitialValue::WORD(create_filler_int_literal(None))
        }
        msg::BaseType::Uint32 if is_dword(field) => {
            dtp::InitialValue::DWORD(create_filler_int_literal(None))
        }
        msg::BaseType::Uint64 if is_lword(field) => {
            dtp::InitialValue::LWORD(create_filler_int_literal(None))
        }
        msg::BaseType::Uint8 => dtp::InitialValue::USINT(create_filler_int_literal(None)),
        msg::BaseType::Uint16 => dtp::InitialValue::UINT(create_filler_int_literal(None)),
        msg::BaseType::Uint32 => dtp::InitialValue::UDINT(create_filler_int_literal(None)),
        msg::BaseType::Uint64 => dtp::InitialValue::ULINT(create_filler_int_literal(None)),
        msg::BaseType::Int8 => dtp::InitialValue::SINT(create_filler_int_literal(None)),
        msg::BaseType::Int16 => dtp::InitialValue::INT(create_filler_int_literal(None)),
        msg::BaseType::Int32 => dtp::InitialValue::DINT(create_filler_int_literal(None)),
        msg::BaseType::Int64 => dtp::InitialValue::LINT(create_filler_int_literal(None)),
        msg::BaseType::Float32 => dtp::InitialValue::REAL(0f32),
        msg::BaseType::Float64 => dtp::InitialValue::LREAL(0f64),
        msg::BaseType::Char => dtp::InitialValue::CHAR(create_filler_char_literal()),
        msg::BaseType::String(_) => dtp::InitialValue::STRING(String::new()),
        msg::BaseType::Wstring(_) => dtp::InitialValue::WSTRING(String::new()),
        msg::BaseType::Custom(_) => unimplemented!(),
    }
}

fn create_filler_char_literal() -> dtp::CharLiteral {
    dtp::CharLiteral::Hex(std::char::from_u32(0u32).expect("casting 0u32 to char literal to work"))
}

fn create_filler_bool_literal(reference_literal: Option<&msg::BoolLiteral>) -> dtp::BoolLiteral {
    match reference_literal {
        Some(msg::BoolLiteral::String(_)) => dtp::BoolLiteral::String(false),
        Some(msg::BoolLiteral::Int(_)) => dtp::BoolLiteral::Int(false),
        None => dtp::BoolLiteral::String(false),
    }
}

fn create_filler_int_literal(reference_literal: Option<&msg::IntLiteral>) -> dtp::IntLiteral {
    match reference_literal {
        Some(msg::IntLiteral::SignedDecimalInt(_)) => dtp::IntLiteral::SignedDecimalInt(0),
        Some(msg::IntLiteral::UnsignedDecimalInt(_)) => dtp::IntLiteral::UnsignedDecimalInt(0),
        Some(msg::IntLiteral::BinaryInt(_)) => dtp::IntLiteral::BinaryInt(0),
        Some(msg::IntLiteral::OctalInt(_)) => dtp::IntLiteral::OctalInt(0),
        Some(msg::IntLiteral::HexalInt(_)) => dtp::IntLiteral::HexalInt(0),
        None => dtp::IntLiteral::UnsignedDecimalInt(0),
    }
}

fn convert_bool_literal(bool_literal: &msg::BoolLiteral) -> dtp::BoolLiteral {
    match bool_literal {
        msg::BoolLiteral::String(bool) => dtp::BoolLiteral::String(*bool),
        msg::BoolLiteral::Int(bool) => dtp::BoolLiteral::Int(*bool),
    }
}

fn convert_to_char_literal(int_literal: &msg::IntLiteral) -> Result<dtp::CharLiteral> {
    Ok(match int_literal {
        msg::IntLiteral::SignedDecimalInt(i64) => dtp::CharLiteral::Hex(i64_to_char(i64)?),
        msg::IntLiteral::UnsignedDecimalInt(u64)
        | msg::IntLiteral::BinaryInt(u64)
        | msg::IntLiteral::OctalInt(u64)
        | msg::IntLiteral::HexalInt(u64) => dtp::CharLiteral::Hex(u64_to_char(u64)?),
    })
}

fn convert_int_literal(int_literal: &msg::IntLiteral) -> dtp::IntLiteral {
    match int_literal {
        msg::IntLiteral::SignedDecimalInt(i64) => dtp::IntLiteral::SignedDecimalInt(*i64),
        msg::IntLiteral::UnsignedDecimalInt(u64) => dtp::IntLiteral::UnsignedDecimalInt(*u64),
        msg::IntLiteral::BinaryInt(u64) => dtp::IntLiteral::BinaryInt(*u64),
        msg::IntLiteral::OctalInt(u64) => dtp::IntLiteral::OctalInt(*u64),
        msg::IntLiteral::HexalInt(u64) => dtp::IntLiteral::HexalInt(*u64),
    }
}

fn i64_to_char(value: &i64) -> Result<char> {
    match char::from_u32(*value as u32) {
        None => Err("The i64 is not a valid Unicode character.".into()),
        Some(c) => Ok(c),
    }
}

fn u64_to_char(value: &u64) -> Result<char> {
    match char::from_u32(*value as u32) {
        None => Err("The u64 is not a valid Unicode character.".into()),
        Some(c) => Ok(c),
    }
}
