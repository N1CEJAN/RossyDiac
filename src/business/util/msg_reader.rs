use log::{debug, info};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until1, take_while1};
use nom::character::complete::{
    anychar, digit1, i16, i32, i64, i8, multispace0, multispace1, u16, u32, u64, u8,
};
use nom::combinator::{map, opt, verify};
use nom::multi::{many0, many_m_n, separated_list0};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

use crate::business::error::Result;
use crate::core::msg::Field::{Constant, Variable};
use crate::core::msg::*;

pub fn read(path_to_file: &str) -> Result<StructuredType> {
    info!("Start reading file {:?}", path_to_file);
    let file_content = std::fs::read_to_string(path_to_file)?;
    let parsed_object = parse(path_to_file, file_content.as_str());
    info!("Finished reading file {:?}", path_to_file);
    parsed_object
}

fn parse(path_to_file: &str, input: &str) -> Result<StructuredType> {
    let (_, fields) = many0(terminated(parse_field, multispace0))(input)?;
    let result = StructuredType::new(path_to_file, fields);
    debug!("parse_file with output: {:#?}", result);
    Ok(result)
}

fn parse_field(input: &str) -> IResult<&str, Field> {
    alt((parse_constant, parse_variable))(input)
}

fn parse_constant(input: &str) -> IResult<&str, Field> {
    map(
        tuple((
            parse_base_type,
            terminated(parse_constraints, multispace1),
            parse_field_name,
            preceded(tag("="), take_until1("\r\n")),
        )),
        |(datatype, constraints, field_name, field_value)| {
            let datatype_copy = datatype.clone();
            let mut parser = parse_field_value(&datatype_copy, &constraints);
            // Annahme: Standardwert wird immer korrekt angegeben
            let value = parser(field_value).unwrap().1;
            Constant(datatype, constraints, field_name.to_string(), value)
        },
    )(input)
}

fn parse_variable(input: &str) -> IResult<&str, Field> {
    map(
        tuple((
            parse_base_type,
            terminated(parse_constraints, multispace1),
            parse_field_name,
            opt(preceded(tag(" "), take_until1("\r\n"))),
        )),
        |(datatype, constraints, field_name, field_value)| {
            let datatype_copy = datatype.clone();
            let mut parser = parse_field_value(&datatype_copy, &constraints);
            // Annahme: Standardwert wird immer korrekt angegeben
            let option = field_value.map(|v| parser(v).unwrap().1);
            Variable(datatype, constraints, field_name.to_string(), option)
        },
    )(input)
}

fn parse_constraints(input: &str) -> IResult<&str, Vec<Constraint>> {
    // Annahme: String Constraints werden nur mit String Datentypen angegeben
    many_m_n(0, 2, parse_constraint)(input)
}

fn parse_base_type(input: &str) -> IResult<&str, BaseType> {
    alt((
        // primitive
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
        map(tag("string"), |_| BaseType::String),
        map(tag("wstring"), |_| BaseType::Wstring),
        // custom
        map(
            alt((take_until1(" "), take_until1("["))),
            |custom_type: &str| BaseType::Custom(custom_type.to_string()),
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
        map(preceded(tag("<="), digit1), |digits: &str| {
            let casted = digits.parse().unwrap();
            Constraint::BoundedString(casted)
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

fn parse_field_value<'a>(
    datatype: &'a BaseType,
    constraints: &[Constraint],
) -> Box<dyn FnMut(&'a str) -> IResult<&'a str, InitialValue> + 'a> {
    let is_array = constraints.iter().any(|c| match c {
        Constraint::StaticArray(_)
        | Constraint::UnboundedDynamicArray
        | Constraint::BoundedDynamicArray(_) => true,
        Constraint::BoundedString(_) => false,
    });

    if is_array {
        Box::new(map(
            delimited(
                tag("["),
                separated_list0(tag(","), parse_field_value(datatype, &[])),
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
            BaseType::Float32 => todo!(),
            BaseType::Float64 => todo!(),
            BaseType::Int8 => Box::new(map(i8, InitialValue::Int8)),
            BaseType::Uint8 => Box::new(map(u8, InitialValue::Uint8)),
            BaseType::Int16 => Box::new(map(i16, InitialValue::Int16)),
            BaseType::Uint16 => Box::new(map(u16, InitialValue::Uint16)),
            BaseType::Int32 => Box::new(map(i32, InitialValue::Int32)),
            BaseType::Uint32 => Box::new(map(u32, InitialValue::Uint32)),
            BaseType::Int64 => Box::new(map(i64, InitialValue::Int64)),
            BaseType::Uint64 => Box::new(map(u64, InitialValue::Uint64)),
            BaseType::Char => Box::new(map(anychar, InitialValue::Char)),
            BaseType::String => Box::new(map(
                // Fehlt: Support für \", wie im Beispiel "Some \"string\""
                // Fehlt: Support für \', wie im Beispiel 'Some \'string\''
                alt((
                    delimited(tag("\""), take_until1("\""), tag("\"")),
                    delimited(tag("'"), take_until1("'"), tag("'")),
                )),
                |str: &str| InitialValue::String(str.to_string()),
            )),
            BaseType::Wstring => Box::new(map(
                // Fehlt: Support für \", wie im Beispiel "Some \"string\""
                // Fehlt: Support für \', wie im Beispiel 'Some \'string\''
                alt((
                    delimited(tag("\""), take_until1("\""), tag("\"")),
                    delimited(tag("'"), take_until1("'"), tag("'")),
                )),
                |str: &str| InitialValue::Wstring(str.to_string()),
            )),
            BaseType::Custom(_) => Box::new(|str| Ok((str, InitialValue::Custom))),
        }
    }
}
