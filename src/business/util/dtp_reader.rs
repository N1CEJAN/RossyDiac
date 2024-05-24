use log::{debug, info};
use xmltree::{Element, XMLNode};

use crate::business::error::{Error, Result};
use crate::core::dtp::*;

pub fn read(path_to_file: &str) -> Result<CustomDataType> {
    info!("Start reading file {:?}", path_to_file);
    let file = std::fs::File::open(path_to_file)?;
    let parsed_object = parse_data_type(file);
    info!("Finished reading file {:?}", path_to_file);
    parsed_object
}

pub fn parse_data_type(file: std::fs::File) -> Result<CustomDataType> {
    let data_type_element = Element::parse(file)?;

    let name = data_type_element
        .attributes
        .get_key_value("Name")
        .map(|key_value| key_value.1.clone())
        .ok_or("No \"Name\" attribute found on \"DataType\" element")?;
    let comment = data_type_element
        .attributes
        .get_key_value("Comment")
        .map(|key_value| key_value.1.clone())
        .unwrap_or("".to_string());
    let data_type_kind = parse_data_type_kind(&data_type_element)?;

    let result = CustomDataType::new(&name, &comment, &data_type_kind);
    debug!("parse_file with output: {:#?}", result);
    Ok(result)
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
        "StructuredType" => Ok(DataTypeKind::StructuredType(parse_structured_type(
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
        .get_key_value("Comment")
        .map(|comment| comment.1.clone())
        .unwrap_or("".to_string());
    let children = parse_structured_type_children(element)?;

    let result = StructuredType::new(&comment, &children);
    debug!("parse_file with output: {:#?}", result);
    Ok(result)
}

fn parse_structured_type_children(element: &Element) -> Result<Vec<StructuredTypeChild>> {
    let structured_type_child_elements = get_filtered_children(element, |child| {
        StructuredTypeChild::matches_any(&child.name)
    });

    let mut result = vec![];
    for structured_type_child_element in structured_type_child_elements.into_iter() {
        match structured_type_child_element.name.as_ref() {
            "VarDeclaration" => result.push(StructuredTypeChild::VarDeclaration(
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
    debug!("parse_structured_type_children with output: {:#?}", result);
    Ok(result)
}

fn parse_var_declaration(element: &Element) -> Result<VarDeclaration> {
    let name = element
        .attributes
        .get_key_value("Name")
        .map(|key_value| key_value.1.clone())
        .ok_or(format!(
            "No \"Name\" attribute defined for \"{}\" element",
            element.name
        ))?;
    let base_type = element
        .attributes
        .get_key_value("Type")
        .map(|key_value| key_value.1.clone())
        .map(|value| parse_base_type(value.as_str()))
        .ok_or(format!(
            "No \"Type\" attribute defined for \"{}\" element",
            element.name
        ))?;
    // sollte definiert sein in Annex B vom IEC 61131-3 (Quelle: IEC 61499-2 Table A.3)
    let array_size = element
        .attributes
        .get_key_value("ArraySize")
        .map(|key_value| key_value.1.clone())
        .map(|value| parse_array_size(value.as_str()))
        .transpose()?;
    // sollte definiert sein in Annex B.1.4.3 vom IEC 61131-3 (Quelle: IEC 61499-2 Table A.3)
    let initial_value = element
        .attributes
        .get_key_value("Comment")
        .map(|key_value| key_value.1.clone())
        .map(|value| parse_initial_value(&base_type, &array_size)(value.as_str()))
        .unwrap_or_else(default_initial_value(&base_type, &array_size))?;
    let comment = element
        .attributes
        .get_key_value("Comment")
        .map(|key_value| key_value.1.clone())
        .unwrap_or("".to_string());

    let result = VarDeclaration::new(&name, &base_type, &array_size, &initial_value, &comment);
    debug!("parse_var_declaration with output: {:#?}", result);
    Ok(result)
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
    if let Some(array_capacity) = array_size {
        // Box::new(move || {
        //     let mut initial_values = Vec::with_capacity(*array_capacity);
        //     for _ in 0..*array_capacity {
        //         initial_values.push(default_initial_value(base_type, &None)())
        //     }
        //     InitialValue::Array(initial_values)
        // })
        todo!()
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
            BaseType::Custom(_) => Box::new(|_| Ok(InitialValue::Custom)),
        }
    }
}

fn default_initial_value<'a>(
    base_type: &'a BaseType,
    array_size: &'a Option<usize>,
) -> Box<dyn FnOnce() -> Result<InitialValue> + 'a> {
    if let Some(array_capacity) = array_size {
        Box::new(move || {
            let mut initial_values = Vec::with_capacity(*array_capacity);
            for _ in 0..*array_capacity {
                initial_values.push(default_initial_value(base_type, &None)()?)
            }
            Ok(InitialValue::Array(initial_values))
        })
    } else {
        match base_type {
            BaseType::BOOL => Box::new(|| Ok(InitialValue::BOOL(false))),
            BaseType::SINT => Box::new(|| Ok(InitialValue::SINT(0))),
            BaseType::INT => Box::new(|| Ok(InitialValue::INT(0))),
            BaseType::DINT => Box::new(|| Ok(InitialValue::DINT(0))),
            BaseType::LINT => Box::new(|| Ok(InitialValue::LINT(0))),
            BaseType::USINT => Box::new(|| Ok(InitialValue::USINT(0))),
            BaseType::UINT => Box::new(|| Ok(InitialValue::UINT(0))),
            BaseType::UDINT => Box::new(|| Ok(InitialValue::UDINT(0))),
            BaseType::ULINT => Box::new(|| Ok(InitialValue::ULINT(0))),
            BaseType::REAL => Box::new(|| Ok(InitialValue::REAL(0.0))),
            BaseType::LREAL => Box::new(|| Ok(InitialValue::LREAL(0.0))),
            BaseType::BYTE => Box::new(|| Ok(InitialValue::BYTE(0))),
            BaseType::WORD => Box::new(|| Ok(InitialValue::WORD(0))),
            BaseType::DWORD => Box::new(|| Ok(InitialValue::DWORD(0))),
            BaseType::LWORD => Box::new(|| Ok(InitialValue::LWORD(0))),
            BaseType::STRING => Box::new(|| Ok(InitialValue::STRING(String::new()))),
            BaseType::WSTRING => Box::new(|| Ok(InitialValue::WSTRING(String::new()))),
            BaseType::TIME => Box::new(|| Ok(InitialValue::TIME(0))),
            BaseType::DATE => Box::new(|| Ok(InitialValue::DATE(0))),
            BaseType::TIME_OF_DAY => Box::new(|| Ok(InitialValue::TIME_OF_DAY(0))),
            BaseType::TOD => Box::new(|| Ok(InitialValue::TOD(0))),
            BaseType::DATE_AND_TIME => Box::new(|| Ok(InitialValue::DATE_AND_TIME(0))),
            BaseType::DT => Box::new(|| Ok(InitialValue::DT(0))),
            BaseType::Custom(_) => Box::new(|| Ok(InitialValue::Custom)),
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
