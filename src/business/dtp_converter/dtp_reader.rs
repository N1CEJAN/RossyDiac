use log::info;
use nom::branch::alt;
use nom::bytes::complete::{is_a, tag, take_till1};
use nom::character::complete::{digit1, hex_digit1, none_of, oct_digit1, one_of};
use nom::combinator::{map, map_res, opt, recognize};
use nom::multi::{many0, many1, many_m_n};
use nom::sequence::{delimited, preceded, tuple};
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

fn parse_data_type(file: std::fs::File) -> Result<DataType> {
    let data_type_element = Element::parse(file)?;
    let name = parse_name(&data_type_element)?;
    let comment = parse_comment(&data_type_element);
    let structured_type = parse_structured_type(&data_type_element)?;
    Ok(DataType::new(name, comment, structured_type))
}

fn parse_structured_type(element: &Element) -> Result<StructuredType> {
    let structured_type_element =
        get_filtered_children(element, |child| child.name == XML_TAG_STRUCTURED_TYPE)
            .into_iter()
            .next()
            .ok_or("XML-Tag \"StructuredType\" expected in XML-Tag \"DataType\"")?;

    let comment = parse_comment(structured_type_element);
    let children = parse_var_declarations(structured_type_element)?;
    Ok(StructuredType::new(comment, children))
}

fn parse_var_declarations(element: &Element) -> Result<Vec<VarDeclaration>> {
    get_filtered_children(element, |child| child.name == XML_TAG_VAR_DECLARATION)
        .into_iter()
        .map(parse_var_declaration)
        .collect::<Result<Vec<_>>>()
}

fn parse_var_declaration(var_declaration_element: &Element) -> Result<VarDeclaration> {
    let name = parse_name(var_declaration_element)?;
    let base_type = var_declaration_element
        .attributes
        .get_key_value(XML_ATTRIBUTE_TYPE)
        .map(|v| v.1.as_str())
        .map(parse_base_type)
        .ok_or("XML-Attribute \"Type\" expected for XML-Tag \"VarDeclaration\"")??;
    let array_size = var_declaration_element
        .attributes
        .get_key_value(XML_ATTRIBUTE_ARRAY_SIZE)
        .map(|key_value| key_value.1.as_str())
        .map(parse_array_size)
        .transpose()?;
    let initial_value = var_declaration_element
        .attributes
        .get_key_value(XML_ATTRIBUTE_INITIAL_VALUE)
        .map(|key_value| key_value.1.as_str())
        .map(|value| parse_initial_value(&base_type, &array_size)(value))
        .transpose()?;
    let comment = parse_comment(var_declaration_element);
    let attributes = parse_attributes(var_declaration_element)?;

    Ok(VarDeclaration::new(
        name,
        base_type,
        array_size,
        initial_value,
        comment,
        attributes,
    ))
}

fn parse_attributes(element: &Element) -> Result<Vec<Attribute>> {
    get_filtered_children(element, |child| child.name == XML_TAG_ATTRIBUTE)
        .into_iter()
        .map(parse_attribute)
        .collect()
}

fn parse_attribute(attribute_element: &Element) -> Result<Attribute> {
    let name = parse_name(attribute_element)?;
    let base_type = attribute_element
        .attributes
        .get_key_value(XML_ATTRIBUTE_TYPE)
        .map(|key_value| key_value.1.clone())
        .map(|value| parse_base_type(value.as_str()))
        .ok_or("No \"Type\" attribute defined for \"Attribute\" element")??;
    let value = attribute_element
        .attributes
        .get_key_value(XML_ATTRIBUTE_VALUE)
        .map(|key_value| key_value.1.clone())
        .map(|value| parse_initial_value(&base_type, &None)(value.as_str()))
        .transpose()?
        .ok_or("No \"Value\" attribute defined for \"Attribute\" element")?;
    let comment = parse_comment(attribute_element);
    Ok(Attribute::new(name, base_type, value, comment))
}

fn parse_base_type(string: &str) -> Result<BaseType> {
    Ok(alt((
        map(tag("BOOL"), |_| BaseType::BOOL),
        map(tag("BYTE"), |_| BaseType::BYTE),
        map(tag("WORD"), |_| BaseType::WORD),
        map(tag("DWORD"), |_| BaseType::DWORD),
        map(tag("LWORD"), |_| BaseType::LWORD),
        map(tag("USINT"), |_| BaseType::USINT),
        map(tag("UINT"), |_| BaseType::UINT),
        map(tag("UDINT"), |_| BaseType::UDINT),
        map(tag("ULINT"), |_| BaseType::ULINT),
        map(tag("SINT"), |_| BaseType::SINT),
        map(tag("INT"), |_| BaseType::INT),
        map(tag("DINT"), |_| BaseType::DINT),
        map(tag("LINT"), |_| BaseType::LINT),
        map(tag("REAL"), |_| BaseType::REAL),
        map(tag("LREAL"), |_| BaseType::LREAL),
        map(tag("CHAR"), |_| BaseType::CHAR),
        map(
            tuple((tag("STRING"), opt(delimited(tag("["), digit1, tag("]"))))),
            |(_, optional_bound): (&str, Option<&str>)| {
                BaseType::STRING(optional_bound.map(|digits| digits.parse().unwrap()))
            },
        ),
        map(
            tuple((tag("WSTRING"), opt(delimited(tag("["), digit1, tag("]"))))),
            |(_, optional_bound): (&str, Option<&str>)| {
                BaseType::WSTRING(optional_bound.map(|digits| digits.parse().unwrap()))
            },
        ),
        map(take_till1(|c| c == '"'), |custom_type: &str| {
            BaseType::Custom(custom_type.to_string())
        }),
    ))(string)
    .map_err(|e: nom::Err<nom::error::Error<&str>>| e.to_owned())
    .finish()?
    .1)
}

fn parse_array_size(input: &str) -> Result<ArraySize> {
    if input.contains("..") {
        let parts: Vec<&str> = input.split("..").collect();
        if parts.len() != 2 {
            return Err("An arrays indexation is expected to match the format 'start..end'".into());
        }
        let start = parts[0].parse().map_err(Error::custom)?;
        let end = parts[1].parse().map_err(Error::custom)?;
        if start > end {
            return Err("An arrays indexation start is expected to be before its end".into());
        }
        Ok(ArraySize::Indexation(start, end))
    } else {
        let capacity = input.parse().map_err(Error::custom)?;
        if capacity == 0 {
            return Err("An arrays capacity is expected to be greater than 0".into());
        }
        Ok(ArraySize::Capacity(capacity))
    }
}

type InitialValueFn<'a> = dyn FnMut(&str) -> Result<InitialValue> + 'a;

fn parse_initial_value<'a>(
    base_type: &'a BaseType,
    array_size: &'a Option<ArraySize>,
) -> Box<InitialValueFn<'a>> {
    if array_size.is_some() {
        Box::new(move |str| {
            let trimmmed = str.trim();
            if !(trimmmed.starts_with('[') && trimmmed.ends_with(']')) {
                return Err("An array must use '[]'".into());
            }
            let values = trimmmed[1..trimmmed.len() - 1]
                .split(',')
                .map(|value| parse_initial_value(base_type, &None)(value.trim()))
                .collect::<Result<Vec<_>>>()?;
            Ok(InitialValue::Array(values))
        })
    } else {
        match base_type {
            BaseType::BOOL => {
                Box::new(|input| parse_bool_representation(input).map(InitialValue::BOOL))
            }
            BaseType::BYTE => {
                Box::new(|input| parse_int_representation(input).map(InitialValue::BYTE))
            }
            BaseType::WORD => {
                Box::new(|input| parse_int_representation(input).map(InitialValue::WORD))
            }
            BaseType::DWORD => {
                Box::new(|input| parse_int_representation(input).map(InitialValue::DWORD))
            }
            BaseType::LWORD => {
                Box::new(|input| parse_int_representation(input).map(InitialValue::LWORD))
            }
            BaseType::USINT => {
                Box::new(|input| parse_int_representation(input).map(InitialValue::USINT))
            }
            BaseType::UINT => {
                Box::new(|input| parse_int_representation(input).map(InitialValue::UINT))
            }
            BaseType::UDINT => {
                Box::new(|input| parse_int_representation(input).map(InitialValue::UDINT))
            }
            BaseType::ULINT => {
                Box::new(|input| parse_int_representation(input).map(InitialValue::ULINT))
            }
            BaseType::SINT => {
                Box::new(|input| parse_int_representation(input).map(InitialValue::SINT))
            }
            BaseType::INT => {
                Box::new(|input| parse_int_representation(input).map(InitialValue::INT))
            }
            BaseType::DINT => {
                Box::new(|input| parse_int_representation(input).map(InitialValue::DINT))
            }
            BaseType::LINT => {
                Box::new(|input| parse_int_representation(input).map(InitialValue::LINT))
            }
            BaseType::REAL => {
                Box::new(|input| parse_real_representation(input).map(InitialValue::REAL))
            }
            BaseType::LREAL => {
                Box::new(|input| parse_lreal_representation(input).map(InitialValue::LREAL))
            }
            BaseType::CHAR => {
                Box::new(|input| parse_char_representation(input).map(InitialValue::CHAR))
            }
            BaseType::STRING(_) => {
                Box::new(|input| parse_string_representation(input).map(InitialValue::STRING))
            }
            BaseType::WSTRING(_) => {
                Box::new(|input| parse_wstring_representation(input).map(InitialValue::WSTRING))
            }
            BaseType::Custom(_) => unreachable!(),
        }
    }
}

fn parse_name(element: &Element) -> Result<String> {
    Ok(element
        .attributes
        .get_key_value(XML_ATTRIBUTE_NAME)
        .map(|key_value| key_value.1.clone())
        .ok_or(format!(
            "XML-Attribute \"Name\" expected on XML-Element \"{}\"",
            element.name
        ))?)
}

fn parse_comment(element: &Element) -> Option<String> {
    element
        .attributes
        .get_key_value(XML_ATTRIBUTE_COMMENT)
        .map(|comment| comment.1.clone())
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

fn parse_bool_representation(input: &str) -> Result<BoolRepresentation> {
    if input == "TRUE" || input == "FALSE" {
        Ok(BoolRepresentation::String(input == "TRUE"))
    } else if input == "1" || input == "0" {
        Ok(BoolRepresentation::Binary(input == "1"))
    } else {
        Err("Invalid boolean literal".into())
    }
}

fn parse_int_representation(input: &str) -> Result<IntRepresentation> {
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

fn parse_real_representation(input: &str) -> Result<f32> {
    input.parse().map_err(Error::custom)
}

fn parse_lreal_representation(input: &str) -> Result<f64> {
    input.parse().map_err(Error::custom)
}

fn parse_char_representation(input: &str) -> Result<CharRepresentation> {
    if !(input.starts_with("'") && input.ends_with("'")) {
        return Err("A char literal is expected to be delimited with single quotes (')".into());
    }
    let actual_value = &input[1..input.len() - 1];

    Ok(char_literal_parser(actual_value)
        .map_err(|e| e.to_owned())
        .finish()?
        .1)
}

fn parse_string_representation(input: &str) -> Result<Vec<CharRepresentation>> {
    if !(input.starts_with("'") && input.ends_with("'")) {
        return Err("InitialValue of STRING must be delimited with ''".into());
    }
    let actual_value = &input[1..input.len() - 1];
    Ok(many0(char_literal_parser)(actual_value)
        .map_err(|e| e.to_owned())
        .finish()?
        .1)
}

fn parse_wstring_representation(input: &str) -> Result<Vec<WcharRepresentation>> {
    if !(input.starts_with("\"") && input.ends_with("\"")) {
        return Err("InitialValue of WSTRING must be delimited with &quot;&quot;".into());
    }
    let actual_value = &input[1..input.len() - 1];
    Ok(many0(wchar_literal_parser)(actual_value)
        .map_err(|e| e.to_owned())
        .finish()?
        .1)
}

fn char_literal_parser(input: &str) -> IResult<&str, CharRepresentation> {
    alt((
        hexadecimal_char_literal_parser,
        map(masked_char_parser, CharRepresentation::Char),
        map(char_parser, CharRepresentation::Char),
    ))(input)
}

fn wchar_literal_parser(input: &str) -> IResult<&str, WcharRepresentation> {
    alt((
        hexadecimal_wchar_literal_parser,
        map(masked_char_parser, WcharRepresentation::Wchar),
        map(char_parser, WcharRepresentation::Wchar),
    ))(input)
}

fn hexadecimal_char_literal_parser(input: &str) -> IResult<&str, CharRepresentation> {
    map_res(
        preceded(tag("$"), recognize(many_m_n(1, 1, hex_digit1))),
        |str| {
            let u8 = u8::from_str_radix(str, 16).map_err(|_| {
                "A hexadecimal char literal is expected \
                to contain a valid 2-digit hexadecimal number"
            })?;
            Ok::<CharRepresentation, Error>(CharRepresentation::Hexadecimal(u8 as char))
        },
    )(input)
}

fn hexadecimal_wchar_literal_parser(input: &str) -> IResult<&str, WcharRepresentation> {
    map_res(
        preceded(tag("$"), recognize(many_m_n(4, 4, hex_digit1))),
        |str| {
            let u16 = u16::from_str_radix(str, 16).map_err(|_| {
                "A hexadecimal wchar literal is expected \
                to contain a valid 4-digit hexadecimal number"
            })?;
            let char = char::try_from(u16 as u32).map_err(|_| {
                "A hexadecimal wchar literal is expected \
                to contain a valid character"
            })?;
            Ok::<WcharRepresentation, Error>(WcharRepresentation::Hexadecimal(char))
        },
    )(input)
}

fn masked_char_parser(input: &str) -> IResult<&str, char> {
    map_res(
        preceded(tag("$"), one_of("LNPTRlnptr$'\"")),
        |char| match char {
            '$' | '"' | '\'' => Ok::<char, Error>(char),
            _ => {
                // TODO: Add support for special characters (=LNPTRlnptr)
                Err("Unsupported masked char literal".into())
            }
        },
    )(input)
}

fn char_parser(input: &str) -> IResult<&str, char> {
    none_of("$'")(input)
}

fn dec_int_parser(input: &str) -> IResult<&str, IntRepresentation> {
    map_res(
        tuple((
            opt(alt((tag("-"), tag("+")))),
            recognize(tuple((digit1, many0(preceded(opt(tag("_")), digit1))))),
        )),
        |(sign, str): (Option<&str>, &str)| {
            let cleaned_str = str.replace("_", "");
            Ok::<IntRepresentation, ParseIntError>(if let Some(sign) = sign {
                let to_parse = format!("{sign}{cleaned_str}");
                let i64 = to_parse.parse::<i64>()?;
                IntRepresentation::SignedDecimal(i64)
            } else {
                let u64 = cleaned_str.parse::<u64>()?;
                IntRepresentation::UnsignedDecimal(u64)
            })
        },
    )(input)
}

fn hex_int_parser(input: &str) -> IResult<&str, IntRepresentation> {
    preceded(
        tag("16#"),
        map_res(
            recognize(many1(preceded(opt(tag("_")), hex_digit1))),
            |str: &str| {
                let cleaned_str = str.replace("_", "");
                let u64 = u64::from_str_radix(&cleaned_str, 16)?;
                Ok::<IntRepresentation, ParseIntError>(IntRepresentation::Heaxdecimal(u64))
            },
        ),
    )(input)
}

fn octal_int_parser(input: &str) -> IResult<&str, IntRepresentation> {
    preceded(
        tag("8#"),
        map_res(
            recognize(many1(preceded(opt(tag("_")), oct_digit1))),
            |str: &str| {
                let cleaned_str = str.replace("_", "");
                let u64 = u64::from_str_radix(&cleaned_str, 8)?;
                Ok::<IntRepresentation, ParseIntError>(IntRepresentation::Octal(u64))
            },
        ),
    )(input)
}

fn bin_int_parser(input: &str) -> IResult<&str, IntRepresentation> {
    preceded(
        tag("2#"),
        map_res(
            recognize(many1(preceded(opt(tag("_")), bin_digit))),
            |str: &str| {
                let cleaned_str = str.replace("_", "");
                let u64 = u64::from_str_radix(&cleaned_str, 2)?;
                Ok::<IntRepresentation, ParseIntError>(IntRepresentation::Binary(u64))
            },
        ),
    )(input)
}

fn bin_digit(input: &str) -> IResult<&str, &str> {
    is_a("01")(input)
}
