use log::{debug, info};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{alphanumeric1, digit1, multispace0};
use nom::combinator::{map, opt, verify};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

use crate::business::error::ServiceError;
use crate::core::parser::msg::field_dto::FieldDto;
use crate::core::parser::msg::field_name_dto::FieldNameDto;
use crate::core::parser::msg::field_type::{
    ArrayConstraint, Constraint, Datatype, FieldType, PrimitiveDatatype, StringConstraint,
    StringDatatype,
};
use crate::core::parser::msg::file_dto::FileDto;

pub fn read(path_to_file: &str) -> Result<FileDto, ServiceError> {
    info!("Start reading file {:?}", path_to_file);
    let file_content = std::fs::read_to_string(path_to_file)
        .map_err(|err| ServiceError::Io(format!("{:?}", err)))?;
    let (_, parsed) = parse_file(file_content.as_str())
        .map_err(|err| ServiceError::Parser(format!("{:?}", err)))?;
    info!("Finished reading file {:?}", path_to_file);
    return Ok(parsed);
}

fn parse_file(input: &str) -> IResult<&str, FileDto> {
    let result = map(many0(terminated(parse_field, multispace0)), FileDto::new)(input)?;
    debug!("parse_file with output: {:?}", result);
    Ok(result)
}

fn parse_field(input: &str) -> IResult<&str, FieldDto> {
    let result = map(
        tuple((parse_field_type, multispace0, parse_field_name)),
        |(field_type, _, field_name)| FieldDto::new(field_type, field_name),
    )(input)?;
    debug!("parse_field with output: {:?}", result.1);
    Ok(result)
}

fn parse_field_type(input: &str) -> IResult<&str, FieldType> {
    let (input, datatype) = alt((
        parse_primitive_datatype,
        parse_string_datatype,
        parse_complex_datatype,
    ))(input)?;
    // Annahme: String Constraints werden nur mit String Datentypen angegeben
    let (input, string_constraint) = opt(parse_string_constraint)(input)?;
    let (input, array_constraint) = opt(parse_array_constraint)(input)?;

    let mut constraints = Vec::with_capacity(2);
    if let Some(string_constraint) = string_constraint {
        constraints.push(Constraint::StringConstraint(string_constraint))
    }
    if let Some(array_constraint) = array_constraint {
        constraints.push(Constraint::ArrayConstraint(array_constraint))
    }
    let result = (input, FieldType::new(datatype, constraints));
    Ok(result)
}

fn parse_primitive_datatype(input: &str) -> IResult<&str, Datatype> {
    let result = map(
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
        )),
        Datatype::Primitive,
    )(input)?;
    Ok(result)
}

fn parse_string_datatype(input: &str) -> IResult<&str, Datatype> {
    let result = map(
        alt((
            map(tag("string"), |_| StringDatatype::String),
            map(tag("wstring"), |_| StringDatatype::Wstring),
        )),
        Datatype::String,
    )(input)?;
    Ok(result)
}

fn parse_complex_datatype(input: &str) -> IResult<&str, Datatype> {
    // https://docs.ros.org/en/foxy/Tutorials/Beginner-Client-Libraries/Creating-Your-First-ROS2-Package.html
    // You can not have nested packages
    let result = map(
        tuple((
            opt(tuple((
                take_while1(|c: char| c.is_alphanumeric() || c == '_'),
                tag("/"),
            ))),
            alphanumeric1,
        )),
        |(prefix, datatype): (Option<(&str, &str)>, &str)| {
            Datatype::Complex(
                prefix.map(|(package, _)| package.to_string()),
                datatype.to_string(),
            )
        },
    )(input)?;
    Ok(result)
}

fn parse_string_constraint(input: &str) -> IResult<&str, StringConstraint> {
    map(preceded(tag("<="), digit1), |digits: &str| {
        let casted = digits.parse::<usize>().unwrap();
        StringConstraint::BoundedString(casted)
    })(input)
}

fn parse_array_constraint(input: &str) -> IResult<&str, ArrayConstraint> {
    // Annahne: N ist in jedem Fall durch usize begrenzt
    let result = alt((
        map(tag("[]"), |_| ArrayConstraint::UnboundedDynamicArray),
        map(delimited(tag("[<="), digit1, tag("]")), |digits: &str| {
            let casted = digits.parse::<usize>().unwrap();
            ArrayConstraint::BoundedDynamicArray(casted)
        }),
        map(delimited(tag("["), digit1, tag("]")), |digits: &str| {
            let casted = digits.parse::<usize>().unwrap();
            ArrayConstraint::StaticArray(casted)
        }),
    ))(input)?;
    Ok(result)
}

fn parse_field_name(input: &str) -> IResult<&str, FieldNameDto> {
    let parser = take_while1(|c: char| c.is_alphanumeric() || c == '_');
    // only if buffer does not have consecutive underscores
    let decorated1 = verify(parser, |s: &str| !s.contains("__"));
    // only if buffer does not end with underscore
    let decorated2 = verify(decorated1, |s: &str| !s.ends_with('_'));
    // only if buffer starts with letter
    let decorated3 = verify(decorated2, |s: &str| {
        s.chars().next().unwrap().is_alphabetic()
    });
    let result = map(decorated3, FieldNameDto::new)(input)?;
    Ok(result)
}
