use crate::business::error::Result;
use crate::core::dtp::{
    ANNOTATION_NAME_ROS2_ABSOLUTE_REFERENCE, ANNOTATION_NAME_ROS2_BOUND_DYNAMIC_ARRAY,
    ANNOTATION_NAME_ROS2_CONSTANT, ANNOTATION_NAME_ROS2_DYNAMIC_ARRAY,
    ANNOTATION_NAME_ROS2_ELEMENT_COUNTER, ANNOTATION_NAME_ROS2_RELATIVE_REFERENCE,
};
use crate::core::msg::{
    ANNOTATION_NAME_IEC61499_WORD, ANNOTATION_NAME_IEC61499_DWORD, 
    ANNOTATION_NAME_IEC61499_LWORD, ANNOTATION_NAME_IEC61499_START_INDEX, 
};
use crate::core::{dtp, msg};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, opt, recognize};
use nom::sequence::{delimited, tuple};
use nom::Finish;

const ELEMENT_COUNTER_SUFFIX: &'static str = "_element_counter";

pub fn convert(package_name: &str, structured_type: &msg::StructuredType) -> Result<dtp::DataType> {
    let name = convert_structured_type_name(package_name, structured_type.name());
    let mut structured_type_children = Vec::new();
    for field in structured_type.fields().into_iter() {
        let children = &mut convert_field(package_name, field)?;
        structured_type_children.append(children)
    }
    let structured_type = dtp::StructuredType::new(None, structured_type_children);
    Ok(dtp::DataType::new(name, None, structured_type))
}

fn convert_structured_type_name(package_name: &str, structured_type_name: &str) -> String {
    let package_name = package_name
        .replace("_", "")
        .replace(" ", "")
        .replace("-", "");
    format!("ROS2_{package_name}_msg_{structured_type_name}")
}

fn convert_field(package_name: &str, field: &msg::Field) -> Result<Vec<dtp::VarDeclaration>> {
    let mut var_declarations = Vec::new();

    let var_name = convert_to_var_name(field)?;
    var_declarations.push(dtp::VarDeclaration::new(
        var_name.clone(),
        convert_to_var_base_type(package_name, field),
        convert_to_dtp_optional_array_size(field)?,
        convert_to_dtp_optional_initial_value(field)?,
        convert_to_var_comment(field)?,
        convert_to_attributes(field)?,
    ));

    if let Some(msg::ArraySize::BoundDynamic(_)) | Some(msg::ArraySize::Dynamic) =
        field.array_size()
    {
        let default_count = compute_element_counter_default_count(field);
        var_declarations.push(dtp::VarDeclaration::new(
            format!("{var_name}{ELEMENT_COUNTER_SUFFIX}"),
            dtp::BaseType::ULINT,
            None,
            Some(dtp::InitialValue::ULINT(
                dtp::IntRepresentation::UnsignedDecimal(default_count),
            )),
            None,
            vec![dtp::Attribute::new(
                ANNOTATION_NAME_ROS2_ELEMENT_COUNTER.to_owned(),
                dtp::BaseType::STRING(None),
                dtp::InitialValue::STRING(
                    field
                        .name()
                        .chars()
                        .map(|char| dtp::CharRepresentation::Char(char))
                        .collect::<Vec<_>>(),
                ),
                None,
            )],
        ));
    };

    Ok(var_declarations)
}

fn convert_to_var_name(field: &msg::Field) -> Result<String> {
    Ok(field.name().to_string())
}

fn convert_to_var_base_type(package_name: &str, field: &msg::Field) -> dtp::BaseType {
    match field.base_type() {
        msg::BaseType::Bool => dtp::BaseType::BOOL,
        msg::BaseType::Byte => dtp::BaseType::BYTE,
        msg::BaseType::Uint16 if is_word(field) => dtp::BaseType::WORD,
        msg::BaseType::Uint32 if is_dword(field) => dtp::BaseType::DWORD,
        msg::BaseType::Uint64 if is_lword(field) => dtp::BaseType::LWORD,
        msg::BaseType::Int8 => dtp::BaseType::SINT,
        msg::BaseType::Int16 => dtp::BaseType::INT,
        msg::BaseType::Int32 => dtp::BaseType::DINT,
        msg::BaseType::Int64 => dtp::BaseType::LINT,
        msg::BaseType::Uint8 => dtp::BaseType::USINT,
        msg::BaseType::Uint16 => dtp::BaseType::UINT,
        msg::BaseType::Uint32 => dtp::BaseType::UDINT,
        msg::BaseType::Uint64 => dtp::BaseType::ULINT,
        msg::BaseType::Float32 => dtp::BaseType::REAL,
        msg::BaseType::Float64 => dtp::BaseType::LREAL,
        msg::BaseType::Char => dtp::BaseType::CHAR,
        msg::BaseType::String(opt_bound) => dtp::BaseType::STRING(opt_bound.clone()),
        msg::BaseType::Wstring(opt_bound) => dtp::BaseType::WSTRING(opt_bound.clone()),
        msg::BaseType::Custom(reference) => {
            dtp::BaseType::Custom(convert_reference(package_name, reference))
        }
    }
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
    if let Some(msg::InitialValue::Array(array)) = field.initial_value() {
        array.len() as u64
    } else {
        0
    }
}

fn convert_to_attributes(field: &msg::Field) -> Result<Vec<dtp::Attribute>> {
    let mut attributes = Vec::new();
    if let msg::BaseType::Custom(msg::Reference::Relative { .. }) = field.base_type() {
        attributes.push(dtp::Attribute::new(
            ANNOTATION_NAME_ROS2_RELATIVE_REFERENCE.to_owned(),
            dtp::BaseType::BOOL,
            dtp::InitialValue::BOOL(dtp::BoolRepresentation::Binary(true)),
            None,
        ))
    }
    if let msg::BaseType::Custom(msg::Reference::Absolute { .. }) = field.base_type() {
        attributes.push(dtp::Attribute::new(
            ANNOTATION_NAME_ROS2_ABSOLUTE_REFERENCE.to_owned(),
            dtp::BaseType::BOOL,
            dtp::InitialValue::BOOL(dtp::BoolRepresentation::Binary(true)),
            None,
        ))
    }
    if let Some(msg::ArraySize::Dynamic) = field.array_size() {
        attributes.push(dtp::Attribute::new(
            ANNOTATION_NAME_ROS2_DYNAMIC_ARRAY.to_owned(),
            dtp::BaseType::BOOL,
            dtp::InitialValue::BOOL(dtp::BoolRepresentation::Binary(true)),
            None,
        ))
    }
    if let Some(msg::ArraySize::BoundDynamic(bound)) = field.array_size() {
        attributes.push(dtp::Attribute::new(
            ANNOTATION_NAME_ROS2_BOUND_DYNAMIC_ARRAY.to_owned(),
            dtp::BaseType::ULINT,
            dtp::InitialValue::ULINT(dtp::IntRepresentation::UnsignedDecimal(*bound as u64)),
            None,
        ))
    }
    if let msg::FieldType::Constant = field.field_type() {
        attributes.push(dtp::Attribute::new(
            ANNOTATION_NAME_ROS2_CONSTANT.to_owned(),
            dtp::BaseType::BOOL,
            dtp::InitialValue::BOOL(dtp::BoolRepresentation::Binary(true)),
            None,
        ))
    }
    Ok(attributes)
}

fn convert_to_dtp_optional_array_size(field: &msg::Field) -> Result<Option<dtp::ArraySize>> {
    Ok(match field.array_size() {
        Some(msg::ArraySize::Capacity(capacity)) if is_shifted_static_array(field) => {
            let start = get_start_index(field)?;
            let end = start + (*capacity as i64) - 1;
            Some(dtp::ArraySize::Indexation(start, end))
        }
        Some(msg::ArraySize::Capacity(capacity)) => Some(dtp::ArraySize::Capacity(*capacity)),
        Some(msg::ArraySize::Dynamic) => Some(dtp::ArraySize::Capacity(3)),
        Some(msg::ArraySize::BoundDynamic(bound)) => Some(dtp::ArraySize::Capacity(*bound)),
        None => None,
    })
}

fn convert_to_dtp_optional_initial_value(field: &msg::Field) -> Result<Option<dtp::InitialValue>> {
    Ok(if let Some(msg_initial_value) = field.initial_value() {
        Some(convert_initial_value(msg_initial_value, field)?)
    } else {
        None
    })
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
        msg::InitialValue::Bool(bool_representation) => {
            dtp::InitialValue::BOOL(convert_bool_representation(bool_representation))
        }
        msg::InitialValue::Byte(int_representation) => {
            dtp::InitialValue::BYTE(convert_int_representation(int_representation))
        }
        msg::InitialValue::Uint8(int_representation) => {
            dtp::InitialValue::USINT(convert_int_representation(int_representation))
        }
        msg::InitialValue::Uint16(int_representation) => {
            dtp::InitialValue::UINT(convert_int_representation(int_representation))
        }
        msg::InitialValue::Uint32(int_representation) => {
            dtp::InitialValue::UDINT(convert_int_representation(int_representation))
        }
        msg::InitialValue::Uint64(int_representation) => {
            dtp::InitialValue::ULINT(convert_int_representation(int_representation))
        }
        msg::InitialValue::Int8(int_representation) => {
            dtp::InitialValue::SINT(convert_int_representation(int_representation))
        }
        msg::InitialValue::Int16(int_representation) => {
            dtp::InitialValue::INT(convert_int_representation(int_representation))
        }
        msg::InitialValue::Int32(int_representation) => {
            dtp::InitialValue::DINT(convert_int_representation(int_representation))
        }
        msg::InitialValue::Int64(int_representation) => {
            dtp::InitialValue::LINT(convert_int_representation(int_representation))
        }
        msg::InitialValue::Float32(f32) => dtp::InitialValue::REAL(*f32),
        msg::InitialValue::Float64(f64) => dtp::InitialValue::LREAL(*f64),
        msg::InitialValue::Char(int_representation) => {
            dtp::InitialValue::CHAR(convert_int_to_char_representation(int_representation)?)
        }
        msg::InitialValue::String(string) => dtp::InitialValue::STRING(
            string
                .chars()
                .map(|char| dtp::CharRepresentation::Char(char))
                .collect::<Vec<_>>(),
        ),
        msg::InitialValue::Wstring(string) => dtp::InitialValue::WSTRING(
            string
                .chars()
                .map(|char| dtp::WcharRepresentation::Wchar(char))
                .collect::<Vec<_>>(),
        ),
        msg::InitialValue::Array(v) => {
            // Determine new capacity based on field constraint
            let new_capacity = match field.array_size() {
                Some(msg::ArraySize::BoundDynamic(capacity)) => *capacity,
                Some(msg::ArraySize::Dynamic) => 3,
                _ => v.len() as u64,
            };

            // Convert initial values
            let mut vec = v
                .iter()
                .map(|value| convert_initial_value(value, field))
                .collect::<Result<Vec<_>>>()?;

            // Add filler values as needed
            if new_capacity > vec.len() as u64 {
                let sample_initial_value = v.iter().next();
                vec.extend(vec![
                    create_filler_initial_value(field, sample_initial_value);
                    new_capacity as usize - vec.len()
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
        || match field.base_type() {
            msg::BaseType::Bool => dtp::InitialValue::BOOL(create_default_bool_representation()),
            msg::BaseType::Byte => dtp::InitialValue::BYTE(create_default_int_representation()),
            msg::BaseType::Uint16 if is_word(field) => dtp::InitialValue::WORD(create_default_int_representation()),
            msg::BaseType::Uint32 if is_dword(field) => dtp::InitialValue::DWORD(create_default_int_representation()),
            msg::BaseType::Uint64 if is_lword(field) => dtp::InitialValue::LWORD(create_default_int_representation()),
            msg::BaseType::Uint8 => dtp::InitialValue::USINT(create_default_int_representation()),
            msg::BaseType::Uint16 => dtp::InitialValue::UINT(create_default_int_representation()),
            msg::BaseType::Uint32 => dtp::InitialValue::UDINT(create_default_int_representation()),
            msg::BaseType::Uint64 => dtp::InitialValue::ULINT(create_default_int_representation()),
            msg::BaseType::Int8 => dtp::InitialValue::SINT(create_default_int_representation()),
            msg::BaseType::Int16 => dtp::InitialValue::INT(create_default_int_representation()),
            msg::BaseType::Int32 => dtp::InitialValue::DINT(create_default_int_representation()),
            msg::BaseType::Int64 => dtp::InitialValue::LINT(create_default_int_representation()),
            msg::BaseType::Float32 => dtp::InitialValue::REAL(create_default_real_representation()),
            msg::BaseType::Float64 => dtp::InitialValue::LREAL(create_default_lreal_representation()),
            msg::BaseType::Char => dtp::InitialValue::CHAR(create_default_char_representation()),
            msg::BaseType::String(_) => dtp::InitialValue::STRING(create_default_string_representation()),
            msg::BaseType::Wstring(_) => dtp::InitialValue::WSTRING(create_default_wstring_representation()),
            msg::BaseType::Custom(_) => unimplemented!(),
        },
        |sample_initial_value| match sample_initial_value {
            msg::InitialValue::Bool(reference) => dtp::InitialValue::BOOL(create_default_bool_representation_from_reference(reference)),
            msg::InitialValue::Byte(reference) => dtp::InitialValue::BYTE(create_default_int_representation_from_reference(reference)),
            msg::InitialValue::Uint16(reference) if is_word(field) => dtp::InitialValue::WORD(create_default_int_representation_from_reference(reference)),
            msg::InitialValue::Uint32(reference) if is_dword(field) => dtp::InitialValue::DWORD(create_default_int_representation_from_reference(reference)),
            msg::InitialValue::Uint64(reference) if is_lword(field) => dtp::InitialValue::LWORD(create_default_int_representation_from_reference(reference)),
            msg::InitialValue::Uint8(reference) => dtp::InitialValue::USINT(create_default_int_representation_from_reference(reference)),
            msg::InitialValue::Uint16(reference) => dtp::InitialValue::UINT(create_default_int_representation_from_reference(reference)),
            msg::InitialValue::Uint32(reference) => dtp::InitialValue::UDINT(create_default_int_representation_from_reference(reference)),
            msg::InitialValue::Uint64(reference) => dtp::InitialValue::ULINT(create_default_int_representation_from_reference(reference)),
            msg::InitialValue::Int8(reference) => dtp::InitialValue::SINT(create_default_int_representation_from_reference(reference)),
            msg::InitialValue::Int16(reference) => dtp::InitialValue::INT(create_default_int_representation_from_reference(reference)),
            msg::InitialValue::Int32(reference) => dtp::InitialValue::DINT(create_default_int_representation_from_reference(reference)),
            msg::InitialValue::Int64(reference) => dtp::InitialValue::LINT(create_default_int_representation_from_reference(reference)),
            msg::InitialValue::Float32(_) => dtp::InitialValue::REAL(create_default_real_representation()),
            msg::InitialValue::Float64(_) => dtp::InitialValue::LREAL(create_default_lreal_representation()),
            msg::InitialValue::Char(_) => dtp::InitialValue::CHAR(create_default_char_representation()),
            msg::InitialValue::String(_) => dtp::InitialValue::STRING(create_default_string_representation()),
            msg::InitialValue::Wstring(_) => dtp::InitialValue::WSTRING(create_default_wstring_representation()),
            msg::InitialValue::Array(_) => unimplemented!(),
        },
    )
}


fn create_default_bool_representation() -> dtp::BoolRepresentation {
    dtp::BoolRepresentation::String(false)
}

fn create_default_int_representation() -> dtp::IntRepresentation {
    dtp::IntRepresentation::UnsignedDecimal(0)
}

fn create_default_real_representation() -> f32 {
    0f32
}

fn create_default_lreal_representation() -> f64 {
    0f64
}

fn create_default_char_representation() -> dtp::CharRepresentation {
    dtp::CharRepresentation::Char('0')
}

fn create_default_string_representation() -> Vec<dtp::CharRepresentation> {
    Vec::new()
}

fn create_default_wstring_representation() -> Vec<dtp::WcharRepresentation> {
    Vec::new()
}

fn create_default_bool_representation_from_reference(
    reference: &msg::BoolRepresentation,
) -> dtp::BoolRepresentation {
    match reference {
        msg::BoolRepresentation::String(_) => dtp::BoolRepresentation::String(false),
        msg::BoolRepresentation::Binary(_) => dtp::BoolRepresentation::Binary(false),
    }
}

fn create_default_int_representation_from_reference(
    reference: &msg::IntRepresentation,
) -> dtp::IntRepresentation {
    match reference {
        msg::IntRepresentation::SignedDecimal(_) => dtp::IntRepresentation::SignedDecimal(0),
        msg::IntRepresentation::UnsignedDecimal(_) => dtp::IntRepresentation::UnsignedDecimal(0),
        msg::IntRepresentation::Binary(_) => dtp::IntRepresentation::Binary(0),
        msg::IntRepresentation::Octal(_) => dtp::IntRepresentation::Octal(0),
        msg::IntRepresentation::Hexadecimal(_) => dtp::IntRepresentation::Heaxdecimal(0),
    }
}

fn convert_bool_representation(
    bool_representation: &msg::BoolRepresentation,
) -> dtp::BoolRepresentation {
    match bool_representation {
        msg::BoolRepresentation::String(bool) => dtp::BoolRepresentation::String(*bool),
        msg::BoolRepresentation::Binary(bool) => dtp::BoolRepresentation::Binary(*bool),
    }
}

fn convert_int_representation(
    int_representation: &msg::IntRepresentation,
) -> dtp::IntRepresentation {
    match int_representation {
        msg::IntRepresentation::SignedDecimal(i64) => dtp::IntRepresentation::SignedDecimal(*i64),
        msg::IntRepresentation::UnsignedDecimal(u64) => {
            dtp::IntRepresentation::UnsignedDecimal(*u64)
        }
        msg::IntRepresentation::Binary(u64) => dtp::IntRepresentation::Binary(*u64),
        msg::IntRepresentation::Octal(u64) => dtp::IntRepresentation::Octal(*u64),
        msg::IntRepresentation::Hexadecimal(u64) => dtp::IntRepresentation::Heaxdecimal(*u64),
    }
}

fn convert_int_to_char_representation(
    int_representation: &msg::IntRepresentation,
) -> Result<dtp::CharRepresentation> {
    Ok(dtp::CharRepresentation::Char(match int_representation {
        msg::IntRepresentation::SignedDecimal(value) => i64_to_char(value),
        msg::IntRepresentation::UnsignedDecimal(value)
        | msg::IntRepresentation::Binary(value)
        | msg::IntRepresentation::Octal(value)
        | msg::IntRepresentation::Hexadecimal(value) => u64_to_char(value),
    }?))
}

fn get_start_index(field: &msg::Field) -> Result<i64> {
    field
        .comment()
        .and_then(|comment| {
            comment
                .find(format!("@{ANNOTATION_NAME_IEC61499_START_INDEX}(").as_str())
                .map(|pos| &comment[pos..])
        })
        .map_or(Ok(0), |input| parse_start_index(input))
}

fn parse_start_index(input: &str) -> Result<i64> {
    Ok(map_res(
        delimited(
            tag(format!("@{ANNOTATION_NAME_IEC61499_START_INDEX}(").as_str()),
            recognize(tuple((opt(alt((tag("-"), tag("+")))), digit1))),
            tag(")"),
        ),
        |str: &str| i64::from_str_radix(str, 10),
    )(input)
    .map_err(|e: nom::Err<nom::error::Error<&str>>| e.to_owned())
    .finish()?
    .1)
}

fn is_word(field: &msg::Field) -> bool {
    field
        .comment()
        .is_some_and(|comment| comment.contains(format!("@{ANNOTATION_NAME_IEC61499_WORD}").as_str()))
}

fn is_dword(field: &msg::Field) -> bool {
    field
        .comment()
        .is_some_and(|comment| comment.contains(format!("@{ANNOTATION_NAME_IEC61499_DWORD}").as_str()))
}

fn is_lword(field: &msg::Field) -> bool {
    field
        .comment()
        .is_some_and(|comment| comment.contains(format!("@{ANNOTATION_NAME_IEC61499_LWORD}").as_str()))
}

fn is_shifted_static_array(field: &msg::Field) -> bool {
    field
        .comment()
        .is_some_and(|comment| comment.contains(format!("@{ANNOTATION_NAME_IEC61499_START_INDEX}").as_str()))
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
