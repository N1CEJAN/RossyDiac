use log::info;
use nom::branch::alt;
use nom::bytes::complete::{is_a, tag};
use nom::character::complete::{digit1, hex_digit1, oct_digit1};
use nom::combinator::{map_res, opt, recognize};
use nom::multi::{many0, many1};
use nom::sequence::{preceded, tuple};
use nom::{Finish, IResult};
use std::num::ParseIntError;
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
    let attributes = parse_attributes(element)?;

    Ok(VarDeclaration::new(
        &name,
        &base_type,
        &array_size,
        &initial_value,
        &comment,
        &attributes,
    ))
}

fn parse_attributes(element: &Element) -> Result<Vec<Attribute>> {
    get_filtered_children(element, |child| child.name == XML_TAG_ATTRIBUTE)
        .into_iter()
        .map(parse_attribute)
        .collect()
}

fn parse_attribute(element: &Element) -> Result<Attribute> {
    let name = element
        .attributes
        .get_key_value(XML_ATTRIBUTE_NAME)
        .map(|key_value| key_value.1.clone())
        .ok_or("No \"Name\" attribute found on \"Attribute\" element")?;
    let base_type = element
        .attributes
        .get_key_value(XML_ATTRIBUTE_TYPE)
        .map(|key_value| key_value.1.clone())
        .map(|value| parse_base_type(value.as_str()))
        .ok_or("No \"Type\" attribute defined for \"Attribute\" element")?;
    let value = element
        .attributes
        .get_key_value(XML_ATTRIBUTE_VALUE)
        .map(|key_value| key_value.1.clone())
        .map(|value| parse_initial_value(&base_type, &None)(value.as_str()))
        .transpose()?
        .ok_or("No \"Value\" attribute defined for \"Attribute\" element")?;
    let comment = element
        .attributes
        .get_key_value(XML_ATTRIBUTE_COMMENT)
        .map(|key_value| key_value.1.clone());

    Ok(Attribute {
        name,
        base_type,
        value,
        comment,
    })
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
            BaseType::BOOL => Box::new(|str| parse_bool_literal(str).map(InitialValue::BOOL)),
            BaseType::SINT => Box::new(|str| parse_int_literal(str).map(InitialValue::SINT)),
            BaseType::INT => Box::new(|str| parse_int_literal(str).map(InitialValue::INT)),
            BaseType::DINT => Box::new(|str| parse_int_literal(str).map(InitialValue::DINT)),
            BaseType::LINT => Box::new(|str| parse_int_literal(str).map(InitialValue::LINT)),
            BaseType::USINT => Box::new(|str| parse_int_literal(str).map(InitialValue::USINT)),
            BaseType::UINT => Box::new(|str| parse_int_literal(str).map(InitialValue::UINT)),
            BaseType::UDINT => Box::new(|str| parse_int_literal(str).map(InitialValue::UDINT)),
            BaseType::ULINT => Box::new(|str| parse_int_literal(str).map(InitialValue::ULINT)),
            BaseType::BYTE => Box::new(|str| parse_int_literal(str).map(InitialValue::BYTE)),
            BaseType::WORD => Box::new(|str| parse_int_literal(str).map(InitialValue::WORD)),
            BaseType::DWORD => Box::new(|str| parse_int_literal(str).map(InitialValue::DWORD)),
            BaseType::LWORD => Box::new(|str| parse_int_literal(str).map(InitialValue::LWORD)),
            BaseType::REAL => parse_primitive_initial_value!(REAL),
            BaseType::LREAL => parse_primitive_initial_value!(LREAL),
            BaseType::CHAR => Box::new(|str| parse_char_literal(str).map(InitialValue::CHAR)),
            BaseType::STRING => Box::new(|str| parse_string_literal(str).map(InitialValue::STRING)),
            BaseType::WSTRING => {
                Box::new(|str| parse_wstring_literal(str).map(InitialValue::WSTRING))
            }
            BaseType::Custom(_) => unreachable!(),
        }
    }
}

fn get_filtered_children(parent: &Element, filter_fn: fn(&Element) -> bool) -> Vec<&Element> {
    get_children(parent)
        .into_iter()
        .filter(|child| filter_fn(child))
        .collect()
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

fn parse_bool_literal(input: &str) -> Result<BoolLiteral> {
    if input == "TRUE" || input == "FALSE" {
        Ok(BoolLiteral::String(input == "TRUE"))
    } else if input == "1" || input == "0" {
        Ok(BoolLiteral::Int(input == "1"))
    } else {
        Err("Invalid boolean literal".into())
    }
}

fn parse_char_literal(input: &str) -> Result<CharLiteral> {
    if !(input.starts_with("'") && input.ends_with("'")) {
        return Err("Input must be delimited with 'value'".into());
    }
    let actual_value = &input[1..input.len() - 1];

    // Annahme: CHAR's werden nur im Format '$<h0><h1>' eingegeben
    if !actual_value.starts_with("$") {
        return Err(
            "Only hex char literals are supported. Hex char literals start with '$'".into(),
        );
    }
    // Strip the '$' prefix
    let hex_part = &actual_value[1..];
    // Parse the remaining part as a hexadecimal number
    let number = u8::from_str_radix(hex_part, 16).map_err(|_| "Invalid hexadecimal number")?;
    match char::from_u32(number as u32) {
        Some(c) => Ok(CharLiteral::Hex(c)),
        None => Err("Invalid char given. UTF-8 hex expected.".into()),
    }
}

fn parse_string_literal(input: &str) -> Result<String> {
    if !(input.starts_with("'") && input.ends_with("'")) {
        return Err("InitialValue of STRING must be delimited with ''".into());
    }
    Ok(input[1..input.len() - 1].to_string())
}

fn parse_wstring_literal(input: &str) -> Result<String> {
    if !(input.starts_with("\"") && input.ends_with("\"")) {
        return Err("InitialValue of WSTRING must be delimited with &quot;&quot;".into());
    }
    Ok(input[1..input.len() - 1].to_string())
}

fn parse_int_literal(input: &str) -> Result<IntLiteral> {
    Ok(alt((
        hex_int_parser,
        octal_int_parser,
        bin_int_parser,
        dec_int_parser,
    ))(input)
    .map_err(|e| e.to_owned())
    .finish()?
    .1)
}

fn dec_int_parser(input: &str) -> IResult<&str, IntLiteral> {
    map_res(
        tuple((
            opt(alt((tag("-"), tag("+")))),
            recognize(tuple((digit1, many0(preceded(opt(tag("_")), digit1))))),
        )),
        |(sign, str): (Option<&str>, &str)| {
            let cleaned_str = str.replace("_", "");
            Ok::<IntLiteral, ParseIntError>(if let Some(sign) = sign {
                let to_parse = format!("{sign}{cleaned_str}");
                let i64 = i64::from_str_radix(&to_parse, 10)?;
                IntLiteral::SignedDecimalInt(i64)
            } else {
                let u64 = u64::from_str_radix(&cleaned_str, 10)?;
                IntLiteral::UnsignedDecimalInt(u64)
            })
        },
    )(input)
}

fn hex_int_parser(input: &str) -> IResult<&str, IntLiteral> {
    preceded(
        tag("16#"),
        map_res(
            recognize(many1(preceded(opt(tag("_")), hex_digit1))),
            |str: &str| {
                let cleaned_str = str.replace("_", "");
                let u64 = u64::from_str_radix(&cleaned_str, 16)?;
                Ok::<IntLiteral, ParseIntError>(IntLiteral::HexalInt(u64))
            },
        ),
    )(input)
}

fn octal_int_parser(input: &str) -> IResult<&str, IntLiteral> {
    preceded(
        tag("8#"),
        map_res(
            recognize(many1(preceded(opt(tag("_")), oct_digit1))),
            |str: &str| {
                let cleaned_str = str.replace("_", "");
                let u64 = u64::from_str_radix(&cleaned_str, 8)?;
                Ok::<IntLiteral, ParseIntError>(IntLiteral::OctalInt(u64))
            },
        ),
    )(input)
}

fn bin_int_parser(input: &str) -> IResult<&str, IntLiteral> {
    preceded(
        tag("2#"),
        map_res(
            recognize(many1(preceded(opt(tag("_")), bin_digit))),
            |str: &str| {
                let cleaned_str = str.replace("_", "");
                let u64 = u64::from_str_radix(&cleaned_str, 2)?;
                Ok::<IntLiteral, ParseIntError>(IntLiteral::BinaryInt(u64))
            },
        ),
    )(input)
}

fn bin_digit(input: &str) -> IResult<&str, &str> {
    is_a("01")(input)
}
