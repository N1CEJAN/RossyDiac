use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{alphanumeric1, digit1, multispace0};
use nom::combinator::{map, verify};
use nom::multi::{many0, many_m_n};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

use crate::business::error::ServiceError;
use crate::core::ros2::field_dto::FieldDto;
use crate::core::ros2::field_name_dto::FieldNameDto;
use crate::core::ros2::field_type::{FieldType, PrimitiveConstraint, PrimitiveDatatype};
use crate::core::ros2::file_dto::FileDto;

pub struct MsgReader;

impl MsgReader {
    pub fn read(path_to_file: &str) -> Result<FileDto, ServiceError> {
        println!("File: {:?}", path_to_file);
        let file_content = std::fs::read_to_string(path_to_file)
            .map_err(|err| ServiceError::Io(format!("{:?}", err)))?;
        println!("File Content: {:?}", file_content);
        let (remaining, parsed) = parse_file(file_content.as_str())
            .map_err(|err| ServiceError::Parser(format!("{:?}", err)))?;
        println!("Remaining: {:?}", remaining);
        return Ok(parsed);
    }
}

fn parse_file(input: &str) -> IResult<&str, FileDto> {
    let result = map(many0(terminated(parse_field, multispace0)), FileDto::new)(input)?;
    println!("parse_file with output: {:?}", result);
    Ok(result)
}

fn parse_field(input: &str) -> IResult<&str, FieldDto> {
    let result = map(
        tuple((parse_field_type, multispace0, parse_field_name)),
        |(field_type, _, field_name)| FieldDto::new(field_type, field_name, None),
    )(input)?;
    println!("parse_field with output: {:?}", result);
    Ok(result)
}

fn parse_field_type(input: &str) -> IResult<&str, FieldType> {
    let result = alt((parse_primitive_fieldtype, parse_complex_fieldtype))(input)?;
    println!("parse_field_type with output: {:?}", result);
    Ok(result)
}

fn parse_primitive_fieldtype(input: &str) -> IResult<&str, FieldType> {
    let result = map(
        tuple((
            parse_primitive_datatype,
            // Annahme: Ein Datentyp kann nur 2 Constraints haben
            // Annahme: Der Constraint BoundedString wird nur mit Strings angegeben
            many_m_n(0, 2, parse_primitive_constraint),
        )),
        |(datatype, constraints)| FieldType::Primitive {
            datatype,
            constraints,
        },
    )(input)?;
    println!("parse_primitive_fieldtype with output: {:?}", result);
    Ok(result)
}

fn parse_primitive_datatype(input: &str) -> IResult<&str, PrimitiveDatatype> {
    let result = alt((
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
    ))(input)?;
    println!("parse_primitive_datatype with output: {:?}", result);
    Ok(result)
}

fn parse_primitive_constraint(input: &str) -> IResult<&str, PrimitiveConstraint> {
    // Annahne: N ist in jedem Fall durch usize begrenzt
    let result = alt((
        map(preceded(tag("<="), digit1), |digits: &str| {
            let casted = digits.parse::<usize>().unwrap();
            PrimitiveConstraint::BoundedString(casted)
        }),
        map(tag("[]"), |_| PrimitiveConstraint::UnboundedDynamicArray),
        map(delimited(tag("["), digit1, tag("]")), |digits: &str| {
            let casted = digits.parse::<usize>().unwrap();
            PrimitiveConstraint::StaticArray(casted)
        }),
        map(delimited(tag("[<="), digit1, tag("]")), |digits: &str| {
            let casted = digits.parse::<usize>().unwrap();
            PrimitiveConstraint::BoundedDynamicArray(casted)
        }),
    ))(input)?;
    println!("parse_primitive_constraint with output: {:?}", result);
    Ok(result)
}

fn parse_complex_fieldtype(input: &str) -> IResult<&str, FieldType> {
    // https://docs.ros.org/en/foxy/Tutorials/Beginner-Client-Libraries/Creating-Your-First-ROS2-Package.html
    // You can not have nested packages
    let result = alt((
        map(
            tuple((
                take_while1(|c: char| c.is_alphanumeric() || c == '_'),
                tag("/"),
                alphanumeric1,
            )),
            |(package, _, datatype): (&str, &str, &str)| FieldType::Complex {
                package: Some(package.to_string().clone()),
                datatype: datatype.to_string().clone(),
            },
        ),
        map(alphanumeric1, |datatype: &str| FieldType::Complex {
            package: None,
            datatype: datatype.to_string().clone(),
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
    println!("parse_field_name with output: {:?}", result);
    Ok(result)
}
