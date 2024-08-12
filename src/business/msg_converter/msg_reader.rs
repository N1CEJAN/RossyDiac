use std::path::Path;

use log::info;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until1, take_while1};
use nom::character::complete::{
    digit1, i16, i32, i64, i8, line_ending, multispace1, u16, u32, u64, u8,
};
use nom::combinator::{eof, map, opt, verify};
use nom::multi::{many0, separated_list0};
use nom::number::complete::{double, float};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::{Finish, IResult};

use crate::business::error::Result;
use crate::core::msg::*;

pub fn read(path_to_source_file: &str) -> Result<StructuredType> {
    info!("Start reading file {:?}", path_to_source_file);
    let source_file_path = Path::new(path_to_source_file);
    let file_name = parse_file_name(source_file_path)?;
    let file_content = std::fs::read_to_string(source_file_path)?;
    let parsed_fields = parse_file(&file_content).finish()?.1;
    let structured_type = StructuredType::new(&file_name, parsed_fields);
    info!("Finished reading file {:?}", path_to_source_file);
    Ok(structured_type)
}

fn parse_file_name(path_to_file: &Path) -> Result<String> {
    let file_name = path_to_file
        .file_stem()
        .map(|os_str| os_str.to_str())
        .flatten()
        .map(|str| str.to_string())
        .ok_or("Could not read file name from file path")?;
    Ok(file_name)
}

fn parse_file(input: &str) -> IResult<&str, Vec<Field>, nom::error::Error<String>> {
    many0(terminated(parse_field, eol_or_eof))(input).map_err(|err| err.to_owned())
}

fn parse_field(input: &str) -> IResult<&str, Field> {
    let (input, (base_type, optional_constraint, name)) = tuple((
        parse_base_type,
        opt(parse_constraint),
        preceded(multispace1, parse_field_name),
    ))(input)?;

    let (input, field_type) = parse_field_type(&base_type, &optional_constraint)(input)?;

    Ok((
        input,
        Field::new(&base_type, &optional_constraint, &name, &field_type),
    ))
}

fn eol_or_eof(input: &str) -> IResult<&str, &str> {
    alt((line_ending, eof))(input)
}

fn parse_base_type(input: &str) -> IResult<&str, BaseType> {
    alt((
        map(tag("bool"), |_| BaseType::Bool),
        map(tag("byte"), |_| BaseType::Byte),
        map(tag("float32"), |_| BaseType::Float32),
        map(tag("float64"), |_| BaseType::Float64),
        map(tag("int8"), |_| BaseType::Int8),
        map(tag("uint8"), |_| BaseType::Uint8),
        map(tag("int16"), |_| BaseType::Int16),
        map(tag("uint16"), |_| BaseType::Uint16),
        map(tag("int32"), |_| BaseType::Int32),
        map(tag("uint32"), |_| BaseType::Uint32),
        map(tag("int64"), |_| BaseType::Int64),
        map(tag("uint64"), |_| BaseType::Uint64),
        map(tag("char"), |_| BaseType::Char),
        map(
            tuple((tag("string"), opt(preceded(tag("<="), digit1)))),
            |(_, optional_bound): (&str, Option<&str>)| {
                BaseType::String(optional_bound.map(|digits| digits.parse().unwrap()))
            },
        ),
        map(tag("wstring"), |_| BaseType::Wstring),
        map(
            alt((take_until1(" "), take_until1("["))),
            |custom_type: &str| {
                BaseType::Custom(Reference::Relative {
                    file: custom_type.to_string(),
                })
            },
        ),
    ))(input)
}

fn parse_constraint(input: &str) -> IResult<&str, Constraint> {
    // Annahne: N ist in jedem Fall durch usize begrenzt
    alt((
        map(tag("[]"), |_| Constraint::UnboundedDynamicArray),
        map(delimited(tag("[<="), digit1, tag("]")), |digits: &str| {
            let casted = digits.parse().unwrap();
            Constraint::BoundedDynamicArray(casted)
        }),
        map(delimited(tag("["), digit1, tag("]")), |digits: &str| {
            let casted = digits.parse().unwrap();
            Constraint::StaticArray(casted)
        }),
    ))(input)
}

fn parse_field_name(input: &str) -> IResult<&str, &str> {
    let parser = take_while1(|c: char| c.is_alphanumeric() || c == '_');
    // only if buffer does not have consecutive underscores
    let decorated1 = verify(parser, |s: &str| !s.contains("__"));
    // only if buffer does not end with underscore
    let decorated2 = verify(decorated1, |s: &str| !s.ends_with('_'));
    // only if buffer starts with letter
    verify(decorated2, |s: &str| {
        s.chars().next().unwrap().is_alphabetic()
    })(input)
}

fn parse_field_type<'a>(
    base_type: &'a BaseType,
    constraint: &'a Option<Constraint>,
) -> impl FnMut(&str) -> IResult<&str, FieldType> + 'a {
    move |input| {
        if let (input, Some(tag)) = opt(alt((tag("="), tag(" "))))(input)? {
            let (input, initial_value) = parse_initial_value(&base_type, &constraint)(input)?;
            let field_type = match tag {
                "=" => FieldType::Constant(initial_value),
                _ => FieldType::Variable(Some(initial_value)),
            };
            Ok((input, field_type))
        } else {
            Ok((input, FieldType::Variable(None)))
        }
    }
}

fn parse_initial_value<'a>(
    datatype: &BaseType,
    optional_constraint: &Option<Constraint>,
) -> Box<dyn FnMut(&'a str) -> IResult<&str, InitialValue> + 'a> {
    if optional_constraint.is_some() {
        Box::new(map(
            delimited(
                tag("["),
                separated_list0(tag(","), parse_initial_value(datatype, &None)),
                tag("]"),
            ),
            InitialValue::Array,
        ))
    } else {
        match datatype {
            BaseType::Bool => Box::new(alt((
                map(tag("true"), |_| InitialValue::Bool(true)),
                map(tag("false"), |_| InitialValue::Bool(false)),
            ))),
            BaseType::Byte => Box::new(map(u8, InitialValue::Byte)),
            BaseType::Float32 => Box::new(map(float, InitialValue::Float32)),
            BaseType::Float64 => Box::new(map(double, InitialValue::Float64)),
            BaseType::Int8 => Box::new(map(i8, InitialValue::Int8)),
            BaseType::Uint8 => Box::new(map(u8, InitialValue::Uint8)),
            BaseType::Int16 => Box::new(map(i16, InitialValue::Int16)),
            BaseType::Uint16 => Box::new(map(u16, InitialValue::Uint16)),
            BaseType::Int32 => Box::new(map(i32, InitialValue::Int32)),
            BaseType::Uint32 => Box::new(map(u32, InitialValue::Uint32)),
            BaseType::Int64 => Box::new(map(i64, InitialValue::Int64)),
            BaseType::Uint64 => Box::new(map(u64, InitialValue::Uint64)),
            // http://design.ros2.org/articles/idl_interface_definition.html
            // A 8-bit single-byte character with a numerical value
            // between 0 and 255 (see 7.2.6.2.1)
            // http://design.ros2.org/articles/generated_interfaces_cpp.html#constructors
            // Constructors: [...](note: char fields are considered numeric for C++).
            BaseType::Char => Box::new(map(u8, InitialValue::Char)),
            // assume defined initial value is correct, due to time constraints
            BaseType::String(_) => Box::new(map(parse_quoted_string, InitialValue::String)),
            BaseType::Wstring => Box::new(map(parse_quoted_string, InitialValue::Wstring)),
            BaseType::Custom(_) => unreachable!(),
        }
    }
}

fn parse_quoted_string(input: &str) -> IResult<&str, String> {
    alt((
        delimited(tag("\""), parse_inner_string('"'), tag("\"")),
        delimited(tag("'"), parse_inner_string('\''), tag("'")),
    ))(input)
}

fn parse_inner_string(quote: char) -> impl FnMut(&str) -> IResult<&str, String> {
    move |input: &str| {
        let mut ret = String::new();
        let mut skip_delimiter = false;
        for (i, ch) in input.char_indices() {
            if ch == '\\' && !skip_delimiter {
                skip_delimiter = true;
            } else if ch == quote && !skip_delimiter {
                return Ok((&input[i..], ret));
            } else {
                ret.push(ch);
                skip_delimiter = false;
            }
        }
        Err(nom::Err::Incomplete(nom::Needed::Unknown))
    }
}
