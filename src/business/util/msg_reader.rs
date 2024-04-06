use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{digit1, line_ending, multispace0};
use nom::combinator::{map, opt, verify};
use nom::IResult;
use nom::multi::{many0, many_m_n};
use nom::sequence::{delimited, preceded, terminated, tuple};

use crate::business::error::ServiceError;
use crate::core::msg::MsgFileDto;

pub struct MsgReader;

impl MsgReader {
    pub fn read(path_to_file: &str) -> Result<MsgFileDto, ServiceError> {
        println!("File: {:?}", path_to_file);
        let file_content = std::fs::read_to_string(path_to_file)
            .map_err(|error| ServiceError::Io(error))?;
        println!("File Content: {:?}", file_content);
        match parse_fields(file_content.as_str()) {
            Ok((_, parsed)) => {
                println!("Output: parsed={:?}", parsed)
            }
            Err(err) => println!("Error: {:?}", err),
        };
        let dto = MsgFileDto::new(file_content.as_str());
        Ok(dto)
    }
}

#[derive(Debug)]
struct Field {
    field_type: FieldType,
    field_name: FieldName,
    field_default_value: Option<String>,
}

#[derive(Debug)]
struct FieldName(String);

impl FieldName {
    fn parse(input: &str) -> IResult<&str, FieldName> {
        let parser1 = take_while1(|c: char| c.is_alphanumeric() || c == '_');
        let parser2 = verify(parser1, |s: &str| !s.contains("__")); // if no consecutive underscores
        let parser3 = verify(parser2, |s: &str| !s.ends_with('_')); // if no underscore ending
        let parser4 = verify(parser3, |s: &str| s.chars().next().unwrap().is_alphabetic()); // if starts with letter
        map(parser4, |parsed: &str| FieldName(parsed.to_string()))(input)
    }
}

#[derive(Debug)]
enum FieldType {
    Primitive { datatype: PrimitiveDatatype, constraints: Vec<PrimitiveConstraint> },
    Complex,
}

#[derive(Debug)]
enum PrimitiveDatatype {
    Bool,
    Byte,
    Char,
    Float32,
    Float64,
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    String,
    Wstring,
}

impl PrimitiveDatatype {
    fn parse(input: &str) -> IResult<&str, PrimitiveDatatype> {
        alt((
            map(tag("bool"), |_| PrimitiveDatatype::Bool),
            map(tag("byte"), |_| PrimitiveDatatype::Byte),
            map(tag("char"), |_| PrimitiveDatatype::Char),
            map(tag("float32"), |_| PrimitiveDatatype::Float32),
            map(tag("float64"), |_| PrimitiveDatatype::Float64),
            map(tag("int8"), |_| PrimitiveDatatype::Int8),
            map(tag("uint8"), |_| PrimitiveDatatype::Uint8),
            map(tag("int16"), |_| PrimitiveDatatype::Int16),
            map(tag("uint16"), |_| PrimitiveDatatype::Uint16),
            map(tag("int32"), |_| PrimitiveDatatype::Int32),
            map(tag("uint32"), |_| PrimitiveDatatype::Uint32),
            map(tag("int64"), |_| PrimitiveDatatype::Int64),
            map(tag("uint64"), |_| PrimitiveDatatype::Uint64),
            map(tag("string"), |_| PrimitiveDatatype::String),
            map(tag("wstring"), |_| PrimitiveDatatype::Wstring),
        ))(input)
    }
}

#[derive(Debug)]
enum PrimitiveConstraint {
    StaticArray(usize),
    UnboundedDynamicArray,
    BoundedDynamicArray(usize),
    BoundedString(usize),
}

impl PrimitiveConstraint {
    fn parse(input: &str) -> IResult<&str, PrimitiveConstraint> {
        // Annahne: N ist in jedem Fall durch usize begrenzt
        alt((
            map(preceded(tag("<="), digit1), |parsed: &str| {
                let casted = parsed.parse::<usize>().unwrap();
                PrimitiveConstraint::BoundedString(casted)
            }),
            map(tag("[]"), |_| PrimitiveConstraint::UnboundedDynamicArray),
            map(delimited(tag("["), digit1, tag("]")), |parsed: &str| {
                let casted = parsed.parse::<usize>().unwrap();
                PrimitiveConstraint::StaticArray(casted)
            }),
            map(delimited(tag("[<="), digit1, tag("]")), |parsed: &str| {
                let casted = parsed.parse::<usize>().unwrap();
                PrimitiveConstraint::BoundedDynamicArray(casted)
            }),
        ))(input)
    }
}

fn parse_fields(input: &str) -> IResult<&str, Vec<Field>> {
    let result = many0(terminated(parse_field, opt(line_ending)))(input)?;
    println!("parse_fields with output: {:?}", result);
    Ok(result)
}

fn parse_field(input: &str) -> IResult<&str, Field> {
    // let (input, (field_type, _, field_name, _, field_default_value, _)) = tuple((
    let parser = tuple((
        parse_field_type, multispace0,
        FieldName::parse, multispace0,
        // parse_field_default_value, multispace0
    ));

    let result = map(parser, |(field_type, _, field_name, _)| {
        Field { field_type, field_name, field_default_value: None }
    })(input)?;
    println!("parse_field with output: {:?}", result);
    Ok(result)
}

fn parse_field_type(input: &str) -> IResult<&str, FieldType> {
    parse_primitive_field_type(input)
}

fn parse_field_default_value(input: &str) -> IResult<&str, &str> {
    todo!()
}

fn parse_primitive_field_type(input: &str) -> IResult<&str, FieldType> {
    let (input, datatype) = PrimitiveDatatype::parse(input)?;
    // Annahme: Ein Datentyp kann nur 2 Constraints haben
    // Annahme: Der Constraint BoundedString wird nur mit Strings angegeben
    let (input, constraints) = many_m_n(0, 2, PrimitiveConstraint::parse)(input)?;
    let result = FieldType::Primitive { datatype, constraints };
    println!("parse_primitive_field_type with output: {:?}", result);
    Ok((input, result))
}

fn parse_complex_field_type(input: &str) -> IResult<&str, &str> {
    todo!()
}

fn parse_static_array(input: &str) -> IResult<&str, &str> {
    todo!()
}

fn parse_unbounded_dynamic_array(input: &str) -> IResult<&str, &str> {
    todo!()
}

fn parse_bounded_dynamic_array(input: &str) -> IResult<&str, &str> {
    todo!()
}

fn parse_bounded_string(input: &str) -> IResult<&str, &str> {
    todo!()
}
