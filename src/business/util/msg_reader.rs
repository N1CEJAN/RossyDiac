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

use crate::business::error::ServiceError;
use crate::core::parser::interface::Field::{Constant, Variable};
use crate::core::parser::interface::{Constraint, Datatype, Field, File, Value};

pub fn read(path_to_file: &str) -> Result<File, ServiceError> {
    info!("Start reading file {:?}", path_to_file);
    let file_content = std::fs::read_to_string(path_to_file)
        .map_err(|err| ServiceError::Io(format!("{:?}", err)))?;
    let (_, parsed) = parse_file(file_content.as_str())
        .map_err(|err| ServiceError::Parser(format!("{:?}", err)))?;
    info!("Finished reading file {:?}", path_to_file);
    return Ok(parsed);
}

fn parse_file(input: &str) -> IResult<&str, File> {
    let result = map(many0(parse_line), File::new)(input)?;
    debug!("parse_file with output: {:#?}", result);
    Ok(result)
}

fn parse_line(input: &str) -> IResult<&str, Field> {
    terminated(alt((parse_constant, parse_variable)), multispace0)(input)
}

fn parse_constant(input: &str) -> IResult<&str, Field> {
    map(
        tuple((
            parse_datatype,
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
            parse_datatype,
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

fn parse_datatype(input: &str) -> IResult<&str, Datatype> {
    alt((
        // primitive
        map(tag("bool"), |_| Datatype::Bool),
        map(tag("byte"), |_| Datatype::Byte),
        map(tag("float32"), |_| Datatype::Float32),
        map(tag("float64"), |_| Datatype::Float64),
        map(tag("int8"), |_| Datatype::Int8),
        map(tag("uint8"), |_| Datatype::Uint8),
        map(tag("int16"), |_| Datatype::Int16),
        map(tag("uint16"), |_| Datatype::Uint16),
        map(tag("int32"), |_| Datatype::Int32),
        map(tag("uint32"), |_| Datatype::Uint32),
        map(tag("int64"), |_| Datatype::Int64),
        map(tag("uint64"), |_| Datatype::Uint64),
        map(tag("char"), |_| Datatype::Char),
        map(tag("string"), |_| Datatype::String),
        map(tag("wstring"), |_| Datatype::Wstring),
        // custom
        map(
            alt((take_until1(" "), take_until1("["))),
            |custom_type: &str| Datatype::Custom(custom_type.to_string()),
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
    datatype: &'a Datatype,
    constraints: &Vec<Constraint>,
) -> Box<dyn FnMut(&'a str) -> IResult<&'a str, Value> + 'a> {
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
                separated_list0(tag(","), parse_field_value(datatype, &vec![])),
                tag("]"),
            ),
            Value::Array,
        ))
    } else {
        match datatype {
            Datatype::Bool => Box::new(alt((
                map(tag("true"), |_| Value::Bool(true)),
                map(tag("false"), |_| Value::Bool(false)),
            ))),
            Datatype::Byte => Box::new(map(u8, Value::Byte)),
            Datatype::Int8 => Box::new(map(i8, Value::Int8)),
            Datatype::Uint8 => Box::new(map(u8, Value::Uint8)),
            Datatype::Int16 => Box::new(map(i16, Value::Int16)),
            Datatype::Uint16 => Box::new(map(u16, Value::Uint16)),
            Datatype::Int32 => Box::new(map(i32, Value::Int32)),
            Datatype::Uint32 => Box::new(map(u32, Value::Uint32)),
            Datatype::Int64 => Box::new(map(i64, Value::Int64)),
            Datatype::Uint64 => Box::new(map(u64, Value::Uint64)),
            Datatype::Float32 => todo!(),
            Datatype::Float64 => todo!(),
            Datatype::Char => Box::new(map(anychar, Value::Char)),
            Datatype::String => Box::new(map(
                // Fehlt: Support für \", wie im Beispiel "Some \"string\""
                // Fehlt: Support für \', wie im Beispiel 'Some \'string\''
                alt((
                    delimited(tag("\""), take_until1("\""), tag("\"")),
                    delimited(tag("'"), take_until1("'"), tag("'")),
                )),
                |str: &str| Value::String(str.to_string()),
            )),
            Datatype::Wstring => Box::new(map(
                // Fehlt: Support für \", wie im Beispiel "Some \"string\""
                // Fehlt: Support für \', wie im Beispiel 'Some \'string\''
                alt((
                    delimited(tag("\""), take_until1("\""), tag("\"")),
                    delimited(tag("'"), take_until1("'"), tag("'")),
                )),
                |str: &str| Value::Wstring(str.to_string()),
            )),
            Datatype::Word
            | Datatype::Dword
            | Datatype::Lword
            | Datatype::Time
            | Datatype::TimeOfDay
            | Datatype::Date
            | Datatype::DateAndTime
            | Datatype::Custom(_) => unimplemented!(),
        }
    }
}
