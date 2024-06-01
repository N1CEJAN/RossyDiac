use log::info;
use xmltree::{Element, XMLNode};

use crate::business::error::{Error, Result};
use crate::core::dtp::*;

pub fn read(path_to_file: &str) -> Result<DataType> {
    info!("Start reading file {:?}", path_to_file);
    let file = std::fs::File::open(path_to_file)?;
    let parsed_object = parse_data_type(file);
    info!("Finished reading file {:?}", path_to_file);
    parsed_object
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
    Ok(VarDeclaration::new(&name, &base_type, &array_size, &initial_value, &comment))
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

fn parse_array_size(input: &str) -> Result<usize> {
    input.parse().map_err(Error::custom)
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
    array_size: &'a Option<usize>,
) -> Box<dyn FnMut(&str) -> Result<InitialValue> + 'a> {
    if let Some(capacity) = array_size {
        Box::new(move |str| {
            let trimmmed = str.trim();
            if !trimmmed.starts_with('{') || !trimmmed.ends_with('}') {
                return Err("Initial value of array types must be delimitied with '{}'".into());
            }
            let inner_str = &trimmmed[1..trimmmed.len() - 1];
            let mut values = Vec::with_capacity(*capacity);
            for asdf in inner_str.split(',') {
                values.push(parse_initial_value(base_type, &None)(asdf)?)
            }
            Ok(InitialValue::Array(values))
        })
    } else {
        match base_type {
            BaseType::BOOL => parse_primitive_initial_value!(BOOL),
            BaseType::SINT => parse_primitive_initial_value!(SINT),
            BaseType::INT => parse_primitive_initial_value!(INT),
            BaseType::DINT => parse_primitive_initial_value!(DINT),
            BaseType::LINT => parse_primitive_initial_value!(LINT),
            BaseType::USINT => parse_primitive_initial_value!(USINT),
            BaseType::UINT => parse_primitive_initial_value!(UINT),
            BaseType::UDINT => parse_primitive_initial_value!(UDINT),
            BaseType::ULINT => parse_primitive_initial_value!(ULINT),
            BaseType::REAL => parse_primitive_initial_value!(REAL),
            BaseType::LREAL => parse_primitive_initial_value!(LREAL),
            BaseType::BYTE => parse_primitive_initial_value!(BYTE),
            BaseType::WORD => parse_primitive_initial_value!(WORD),
            BaseType::DWORD => parse_primitive_initial_value!(DWORD),
            BaseType::LWORD => parse_primitive_initial_value!(LWORD),
            BaseType::STRING => parse_primitive_initial_value!(STRING),
            BaseType::WSTRING => parse_primitive_initial_value!(WSTRING),
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
