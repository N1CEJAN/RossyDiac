use log::info;
use nom::branch::alt;
use nom::bytes::complete::{is_a, tag};
use nom::character::complete::{digit1, hex_digit1, oct_digit1};
use nom::combinator::{map, opt, recognize};
use nom::multi::{many0, many1};
use nom::sequence::{preceded, terminated, tuple};
use nom::{Finish, IResult};
use xmltree::{Element, XMLNode};

use crate::business::error::{Error, Result};
use crate::core::dtp::*;

pub fn read(path_to_file: &str) -> Result<DataType> {
    info!("Start reading file {:?}", path_to_file);
    let file = std::fs::File::open(path_to_file)?;
    let data_type = parse_data_type(file)?;
    info!("Finished reading file {:?}", path_to_file);
    Ok(data_type)
}

pub fn parse_data_type(file: std::fs::File) -> Result<DataType> {
    let data_type_element = Element::parse(file)?;
    let name = data_type_element
        .attributes
        .get_key_value(XML_ATTRIBUTE_NAME)
        .map(|key_value| key_value.1.clone())
        .ok_or("No \"Name\" attribute found on \"DataType\" element")?;
    let comment = data_type_element
        .attributes
        .get_key_value(XML_ATTRIBUTE_COMMENT)
        .map(|key_value| key_value.1.clone());
    let data_type_kind = parse_data_type_kind(&data_type_element)?;
    Ok(DataType::new(&name, &comment, &data_type_kind))
}

fn parse_data_type_kind(element: &Element) -> Result<DataTypeKind> {
    let data_type_kind_element =
        get_filtered_children(element, |child| DataTypeKind::matches_any(&child.name))
            .into_iter()
            .next()
            .ok_or("No data type kind element found in \"DataType\" element")?;

    match data_type_kind_element.name.as_ref() {
        // "DirectlyDerivedType" => Ok(DataTypeKind::DirectlyDerivedType(parse_directly_derived_type(
        //     &data_type_kind_element,
        // )?)),
        // "EnumeratedType" => Ok(DataTypeKind::EnumeratedType(parse_enumerated_type(
        //     &data_type_kind_element,
        // )?)),
        // "SubrangeType" => Ok(DataTypeKind::SubrangeType(parse_subrange_type(
        //     &data_type_kind_element,
        // )?)),
        // "ArrayType" => Ok(DataTypeKind::ArrayType(parse_array_type(
        //     &data_type_kind_element,
        // )?)),
        XML_TAG_STRUCTURED_TYPE => Ok(DataTypeKind::StructuredType(parse_structured_type(
            data_type_kind_element,
        )?)),
        _ => Err(format!(
            "Unsupported \"DataType\" child element: {}",
            data_type_kind_element.name
        )
        .into()),
    }
}

fn parse_structured_type(element: &Element) -> Result<StructuredType> {
    let comment = element
        .attributes
        .get_key_value(XML_ATTRIBUTE_COMMENT)
        .map(|comment| comment.1.clone());
    let children = parse_structured_type_children(element)?;
    Ok(StructuredType::new(&comment, &children))
}

fn parse_structured_type_children(element: &Element) -> Result<Vec<StructuredTypeChild>> {
    let structured_type_child_elements = get_filtered_children(element, |child| {
        StructuredTypeChild::matches_any(&child.name)
    });

    let mut result = vec![];
    for structured_type_child_element in structured_type_child_elements.into_iter() {
        match structured_type_child_element.name.as_ref() {
            XML_TAG_VAR_DECLARATION => result.push(StructuredTypeChild::VarDeclaration(
                parse_var_declaration(structured_type_child_element)?,
            )),
            // "SubrangeVarDeclaration" => result.push(StructuredTypeChild::SubrangeVarDeclaration(
            //     parse_subrange_var_declaration(element),
            // )),
            _ => {
                return Err(format!(
                    "Unsupported StructuredType child element : {}",
                    element.name
                )
                .into());
            }
        };
    }
    Ok(result)
}

fn parse_var_declaration(element: &Element) -> Result<VarDeclaration> {
    let name = element
        .attributes
        .get_key_value(XML_ATTRIBUTE_NAME)
        .map(|key_value| key_value.1.clone())
        .ok_or(format!(
            "No \"Name\" attribute defined for \"{}\" element",
            element.name
        ))?;
    let base_type = element
        .attributes
        .get_key_value(XML_ATTRIBUTE_TYPE)
        .map(|key_value| key_value.1.clone())
        .map(|value| parse_base_type(value.as_str()))
        .ok_or(format!(
            "No \"Type\" attribute defined for \"{}\" element",
            element.name
        ))?;
    // sollte definiert sein in Annex B vom IEC 61131-3 (Quelle: IEC 61499-2 Table A.3)
    let array_size = element
        .attributes
        .get_key_value(XML_ATTRIBUTE_ARRAY_SIZE)
        .map(|key_value| key_value.1.clone())
        .map(|value| parse_array_size(value.as_str()))
        .transpose()?;
    // sollte definiert sein in Annex B.1.4.3 vom IEC 61131-3 (Quelle: IEC 61499-2 Table A.3)
    let initial_value = element
        .attributes
        .get_key_value(XML_ATTRIBUTE_INITIAL_VALUE)
        .map(|key_value| key_value.1.clone())
        .map(|value| parse_initial_value(&base_type, &array_size)(value.as_str()))
        .transpose()?;
    let comment = element
        .attributes
        .get_key_value(XML_ATTRIBUTE_COMMENT)
        .map(|key_value| key_value.1.clone());
    Ok(VarDeclaration::new(
        &name,
        &base_type,
        &array_size,
        &initial_value,
        &comment,
    ))
}

fn parse_base_type(string: &str) -> BaseType {
    match string {
        "BOOL" => BaseType::BOOL,
        "SINT" => BaseType::SINT,
        "INT" => BaseType::INT,
        "DINT" => BaseType::DINT,
        "LINT" => BaseType::LINT,
        "USINT" => BaseType::USINT,
        "UINT" => BaseType::UINT,
        "UDINT" => BaseType::UDINT,
        "ULINT" => BaseType::ULINT,
        "REAL" => BaseType::REAL,
        "LREAL" => BaseType::LREAL,
        "BYTE" => BaseType::BYTE,
        "WORD" => BaseType::WORD,
        "DWORD" => BaseType::DWORD,
        "LWORD" => BaseType::LWORD,
        "CHAR" => BaseType::CHAR,
        "STRING" => BaseType::STRING,
        "WSTRING" => BaseType::WSTRING,
        "TIME" => BaseType::TIME,
        "DATE" => BaseType::DATE,
        "TIME_OF_DAY" => BaseType::TIME_OF_DAY,
        "TOD" => BaseType::TOD,
        "DATE_AND_TIME" => BaseType::DATE_AND_TIME,
        "DT" => BaseType::DT,
        _ => BaseType::Custom(string.to_string()),
    }
}

fn parse_array_size(input: &str) -> Result<ArraySize> {
    if input == "*" {
        Ok(ArraySize::Dynamic)
    } else {
        if input.contains("..") {
            let parts: Vec<&str> = input.split("..").collect();
            if parts.len() != 2 {
                return Err("Input must be in the format 'start..end' or 'capacity'".into());
            }
            let start = parts[0].parse().map_err(|_| "Invalid start value")?;
            let end = parts[1].parse().map_err(|_| "Invalid end value")?;
            Ok(ArraySize::Static(Capacity::Shifted(start, end)))
        } else {
            let capacity = input.parse().map_err(Error::custom)?;
            Ok(ArraySize::Static(Capacity::InPlace(capacity)))
        }
    }
}

macro_rules! parse_primitive_initial_value {
    ($iec61131_primitive:ident) => {
        Box::new(|str| {
            str.parse()
                .map(InitialValue::$iec61131_primitive)
                .map_err(Error::custom)
        })
    };
}

fn parse_initial_value<'a>(
    base_type: &'a BaseType,
    array_size: &'a Option<ArraySize>,
) -> Box<dyn FnMut(&str) -> Result<InitialValue> + 'a> {
    if let Some(array_size) = array_size {
        Box::new(move |str| {
            let trimmmed = str.trim();
            if !(trimmmed.starts_with('[') && trimmmed.ends_with(']')) {
                return match array_size {
                    ArraySize::Dynamic => Ok(InitialValue::Array(Vec::new())),
                    ArraySize::Static(_) => Err("Static arrays must use '[]'".into()),
                };
            }
            let values = trimmmed[1..trimmmed.len() - 1]
                .split(',')
                .map(|value| parse_initial_value(base_type, &None)(value.trim()))
                .collect::<Result<Vec<_>>>()?;
            Ok(InitialValue::Array(values))
        })
    } else {
        match base_type {
            BaseType::BOOL => Box::new(|str| parse_bool_initial_value(str).map(InitialValue::BOOL)),
            BaseType::SINT => Box::new(|str| parse_sint_initial_value(str)),
            BaseType::INT => parse_primitive_initial_value!(INT),
            BaseType::DINT => parse_primitive_initial_value!(DINT),
            BaseType::LINT => parse_primitive_initial_value!(LINT),
            BaseType::USINT => parse_primitive_initial_value!(USINT),
            BaseType::UINT => parse_primitive_initial_value!(UINT),
            BaseType::UDINT => parse_primitive_initial_value!(UDINT),
            BaseType::ULINT => parse_primitive_initial_value!(ULINT),
            BaseType::REAL => parse_primitive_initial_value!(REAL),
            BaseType::LREAL => parse_primitive_initial_value!(LREAL),
            BaseType::BYTE => Box::new(|str| {
                parse_byte_string_initial_value(str)
                    .map(|v| v as u8)
                    .map(InitialValue::BYTE)
            }),
            BaseType::WORD => Box::new(|str| {
                parse_byte_string_initial_value(str)
                    .map(|v| v as u16)
                    .map(InitialValue::WORD)
            }),
            BaseType::DWORD => Box::new(|str| {
                parse_byte_string_initial_value(str)
                    .map(|v| v as u32)
                    .map(InitialValue::DWORD)
            }),
            BaseType::LWORD => Box::new(|str| {
                parse_byte_string_initial_value(str)
                    .map(|v| v as u64)
                    .map(InitialValue::LWORD)
            }),
            BaseType::CHAR => Box::new(|str| parse_char_initial_value(str).map(InitialValue::CHAR)),
            BaseType::STRING => {
                Box::new(|str| parse_string_initial_value(str).map(InitialValue::STRING))
            }
            BaseType::WSTRING => {
                Box::new(|str| parse_wstring_initial_value(str).map(InitialValue::WSTRING))
            }
            BaseType::TIME => parse_primitive_initial_value!(TIME),
            BaseType::DATE => parse_primitive_initial_value!(DATE),
            BaseType::TIME_OF_DAY => parse_primitive_initial_value!(TIME_OF_DAY),
            BaseType::TOD => parse_primitive_initial_value!(TOD),
            BaseType::DATE_AND_TIME => parse_primitive_initial_value!(DATE_AND_TIME),
            BaseType::DT => parse_primitive_initial_value!(DT),
            BaseType::Custom(_) => unreachable!(),
        }
    }
}

fn get_filtered_children(parent: &Element, x: fn(&&Element) -> bool) -> Vec<&Element> {
    get_children(parent).into_iter().filter(x).collect()
}

fn get_children(parent: &Element) -> Vec<&Element> {
    parent
        .children
        .iter()
        .map(XMLNode::as_element)
        .filter(Option::is_some)
        .flatten()
        .collect()
}

fn parse_bool_initial_value(input: &str) -> Result<bool> {
    input.to_lowercase().parse().map_err(Error::custom)
}

fn parse_byte_string_initial_value(input: &str) -> Result<u64> {
    let parts: Vec<&str> = input.split('#').collect();
    if parts.len() != 2 {
        return Err("InitialValue of byte string expected to be a format of 'base#number'".into());
    }
    u64::from_str_radix(parts[1], 16).map_err(Error::custom)
}

fn parse_char_initial_value(input: &str) -> Result<u8> {
    if !(input.starts_with("'") && input.ends_with("'")) {
        return Err("Input must be delimited with 'value'".into());
    }
    let actual_value = &input[1..input.len() - 1];

    // Annahme: CHAR's werden nur im Format '$<h0><h1>' eingegeben
    if !actual_value.starts_with("$") {
        return Err("Input must start with '$'".into());
    }
    // Strip the '$' prefix
    let hex_part = &actual_value[1..];
    // Parse the remaining part as a hexadecimal number
    let number =
        u8::from_str_radix(hex_part, 16).map_err(|_| "Invalid hexadecimal number".to_string())?;

    Ok(number)
}

fn parse_string_initial_value(input: &str) -> Result<String> {
    if !(input.starts_with("'") && input.ends_with("'")) {
        return Err("InitialValue of STRING must be delimited with ''".into());
    }
    Ok(input[1..input.len() - 1].to_string())
}

fn parse_wstring_initial_value(input: &str) -> Result<String> {
    if !(input.starts_with("\"") && input.ends_with("\"")) {
        return Err("InitialValue of WSTRING must be delimited with &quot;&quot;".into());
    }
    Ok(input[1..input.len() - 1].to_string())
}

fn parse_sint_initial_value(input: &str) -> Result<InitialValue> {
    let literal = parse_int_literal(input)
        .map_err(|e| e.to_owned())
        .finish()?
        .1;
    match literal.int_type {
        Some(IntTypeName::SignedIntTypeName(SignedIntTypeName::SINT)) | None => {}
        _ => return Err("Invalid integer type name found".into()),
    }
    Ok(InitialValue::SINT(literal))
}

fn parse_int_literal(input: &str) -> IResult<&str, IntLiteral> {
    map(
        tuple((
            opt(terminated(int_type_name_parser, tag("#"))),
            alt((
                hex_int_parser,
                octal_int_parser,
                binary_int_parser,
                dec_int_parser,
            )),
        )),
        |(int_type, (str, e_int_literal))| IntLiteral {
            int_type,
            value: str.to_string(),
            e_int_literal,
        },
    )(input)
}

fn dec_int_parser(input: &str) -> IResult<&str, (&str, EIntLiteral)> {
    map(
        recognize(tuple((
            opt(alt((tag("-"), tag("+")))),
            tuple((digit1, many0(preceded(opt(tag("_")), digit1)))),
        ))),
        |str: &str| (str, EIntLiteral::DecimalInt),
    )(input)
}

fn hex_int_parser(input: &str) -> IResult<&str, (&str, EIntLiteral)> {
    preceded(
        tag("16#"),
        map(
            recognize(many1(preceded(opt(tag("_")), hex_digit1))),
            |str| (str, EIntLiteral::HexalInt),
        ),
    )(input)
}

fn octal_int_parser(input: &str) -> IResult<&str, (&str, EIntLiteral)> {
    preceded(
        tag("8#"),
        map(
            recognize(many1(preceded(opt(tag("_")), oct_digit1))),
            |str| (str, EIntLiteral::OctalInt),
        ),
    )(input)
}

fn binary_int_parser(input: &str) -> IResult<&str, (&str, EIntLiteral)> {
    preceded(
        tag("2#"),
        map(
            recognize(many1(preceded(opt(tag("_")), bin_digit))),
            |str| (str, EIntLiteral::BinaryInt),
        ),
    )(input)
}

fn int_type_name_parser(input: &str) -> IResult<&str, IntTypeName> {
    alt((
        map(signed_int_type_name_parser, IntTypeName::SignedIntTypeName),
        map(
            unsigned_int_type_name_parser,
            IntTypeName::UnsignedIntTypeName,
        ),
    ))(input)
}

fn signed_int_type_name_parser(input: &str) -> IResult<&str, SignedIntTypeName> {
    alt((
        map(tag("SINT"), |_| SignedIntTypeName::SINT),
        map(tag("INT"), |_| SignedIntTypeName::INT),
        map(tag("DINT"), |_| SignedIntTypeName::DINT),
        map(tag("LINT"), |_| SignedIntTypeName::LINT),
    ))(input)
}

fn unsigned_int_type_name_parser(input: &str) -> IResult<&str, UnsignedIntTypeName> {
    alt((
        map(tag("USINT"), |_| UnsignedIntTypeName::USINT),
        map(tag("UINT"), |_| UnsignedIntTypeName::UINT),
        map(tag("UDINT"), |_| UnsignedIntTypeName::UDINT),
        map(tag("ULINT"), |_| UnsignedIntTypeName::ULINT),
    ))(input)
}

fn bin_digit(input: &str) -> IResult<&str, &str> {
    is_a("01")(input)
}
