use log::{debug, info};
use xmltree::{Element, XMLNode};

use crate::business::error::ServiceError;
use crate::core::dtp::{
    BaseType, CustomDataType, DataTypeKind, InitialValue, PrimitiveDataType, PrimitiveValue,
    StructuredType, StructuredTypeChild, VarDeclaration,
};

pub fn read(path_to_file: &str) -> Result<CustomDataType, ServiceError> {
    info!("Start reading file {:?}", path_to_file);
    let file =
        std::fs::File::open(path_to_file).map_err(|err| ServiceError::Io(format!("{:?}", err)))?;
    let parsed_object = parse_data_type(file);
    info!("Finished reading file {:?}", path_to_file);
    parsed_object
}

pub fn parse_data_type(file: std::fs::File) -> Result<CustomDataType, ServiceError> {
    let data_type_element =
        Element::parse(file).map_err(|err| ServiceError::Parser(format!("{:?}", err)))?;

    let name = data_type_element
        .attributes
        .get_key_value("Name")
        .map(|key_value| key_value.1.clone())
        .ok_or(ServiceError::Parser(String::from(
            "No \"Name\" attribute found on \"DataType\" element",
        )))?;
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

fn parse_data_type_kind(element: &Element) -> Result<DataTypeKind, ServiceError> {
    let data_type_kind_element =
        get_filtered_children(&element, |child| DataTypeKind::matches_any(&child.name))
            .into_iter()
            .nth(0)
            .ok_or(ServiceError::Parser(String::from(
                "No element found defining the data type kind in \"DataType\" element",
            )))?;

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
            &data_type_kind_element,
        )?)),
        _ => Err(ServiceError::Parser(format!(
            "Unsupported DataType child element: {}",
            data_type_kind_element.name
        ))),
    }
}

fn parse_structured_type(element: &Element) -> Result<StructuredType, ServiceError> {
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

fn parse_structured_type_children(
    element: &Element,
) -> Result<Vec<StructuredTypeChild>, ServiceError> {
    let structured_type_child_elements = get_filtered_children(&element, |child| {
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
                return Err(ServiceError::Parser(format!(
                    "Unsupported StructuredType child element : {}",
                    element.name
                )));
            }
        };
    }
    debug!("parse_structured_type_children with output: {:#?}", result);
    Ok(result)
}

fn parse_var_declaration(element: &Element) -> Result<VarDeclaration, ServiceError> {
    let name = element
        .attributes
        .get_key_value("Name")
        .map(|key_value| key_value.1.clone())
        .ok_or(ServiceError::Parser(format!(
            "No \"Name\" attribute defined for \"{}\" element",
            element.name
        )))?;
    let base_type = element
        .attributes
        .get_key_value("Type")
        .map(|key_value| key_value.1.clone())
        .map(|value| parse_base_type(value.as_str()))
        .ok_or(ServiceError::Parser(format!(
            "No \"Type\" attribute defined for \"{}\" element",
            element.name
        )))?;
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
        "BOOL" => BaseType::Primitive(PrimitiveDataType::BOOL),
        "SINT" => BaseType::Primitive(PrimitiveDataType::SINT),
        "INT" => BaseType::Primitive(PrimitiveDataType::INT),
        "DINT" => BaseType::Primitive(PrimitiveDataType::DINT),
        "LINT" => BaseType::Primitive(PrimitiveDataType::LINT),
        "USINT" => BaseType::Primitive(PrimitiveDataType::USINT),
        "UINT" => BaseType::Primitive(PrimitiveDataType::UINT),
        "UDINT" => BaseType::Primitive(PrimitiveDataType::UDINT),
        "ULINT" => BaseType::Primitive(PrimitiveDataType::ULINT),
        "REAL" => BaseType::Primitive(PrimitiveDataType::REAL),
        "LREAL" => BaseType::Primitive(PrimitiveDataType::LREAL),
        "BYTE" => BaseType::Primitive(PrimitiveDataType::BYTE),
        "WORD" => BaseType::Primitive(PrimitiveDataType::WORD),
        "DWORD" => BaseType::Primitive(PrimitiveDataType::DWORD),
        "LWORD" => BaseType::Primitive(PrimitiveDataType::LWORD),
        "STRING" => BaseType::Primitive(PrimitiveDataType::STRING),
        "WSTRING" => BaseType::Primitive(PrimitiveDataType::WSTRING),
        "TIME" => BaseType::Primitive(PrimitiveDataType::TIME),
        "DATE" => BaseType::Primitive(PrimitiveDataType::DATE),
        "TIME_OF_DAY" => BaseType::Primitive(PrimitiveDataType::TIME_OF_DAY),
        "TOD" => BaseType::Primitive(PrimitiveDataType::TOD),
        "DATE_AND_TIME" => BaseType::Primitive(PrimitiveDataType::DATE_AND_TIME),
        "DT" => BaseType::Primitive(PrimitiveDataType::DT),
        _ => BaseType::Custom(string.to_string()),
    }
}

fn parse_array_size(input: &str) -> Result<usize, ServiceError> {
    input
        .parse()
        .map_err(|err| ServiceError::Parser(format!("{:?}", err)))
}

fn parse_initial_value<'a>(
    base_type: &'a BaseType,
    array_size: &'a Option<usize>,
) -> Box<dyn FnMut(&str) -> Result<InitialValue, ServiceError> + 'a> {
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
            BaseType::Primitive(primitive) => Box::new(move |str| {
                Ok(InitialValue::Primitive(parse_primitive_initial_value(
                    primitive,
                )(str)?))
            }),
            BaseType::Custom(_) => Box::new(|_| Ok(InitialValue::Custom)),
        }
    }
}

macro_rules! parse_primitive_initial_value {
    ($iec61131_primitive:ident) => {
        Box::new(|str| {
            str.parse()
                .map(PrimitiveValue::$iec61131_primitive)
                .map_err(|err| ServiceError::Parser(format!("{:?}", err)))
        })
    };
}
fn parse_primitive_initial_value(
    primitive_data_type: &PrimitiveDataType,
) -> Box<dyn FnMut(&str) -> Result<PrimitiveValue, ServiceError>> {
    match primitive_data_type {
        PrimitiveDataType::BOOL => parse_primitive_initial_value!(BOOL),
        PrimitiveDataType::SINT => parse_primitive_initial_value!(SINT),
        PrimitiveDataType::INT => parse_primitive_initial_value!(INT),
        PrimitiveDataType::DINT => parse_primitive_initial_value!(DINT),
        PrimitiveDataType::LINT => parse_primitive_initial_value!(LINT),
        PrimitiveDataType::USINT => parse_primitive_initial_value!(USINT),
        PrimitiveDataType::UINT => parse_primitive_initial_value!(UINT),
        PrimitiveDataType::UDINT => parse_primitive_initial_value!(UDINT),
        PrimitiveDataType::ULINT => parse_primitive_initial_value!(ULINT),
        PrimitiveDataType::REAL => parse_primitive_initial_value!(REAL),
        PrimitiveDataType::LREAL => parse_primitive_initial_value!(LREAL),
        PrimitiveDataType::BYTE => parse_primitive_initial_value!(BYTE),
        PrimitiveDataType::WORD => parse_primitive_initial_value!(WORD),
        PrimitiveDataType::DWORD => parse_primitive_initial_value!(DWORD),
        PrimitiveDataType::LWORD => parse_primitive_initial_value!(LWORD),
        PrimitiveDataType::STRING => parse_primitive_initial_value!(STRING),
        PrimitiveDataType::WSTRING => parse_primitive_initial_value!(WSTRING),
        PrimitiveDataType::TIME => parse_primitive_initial_value!(TIME),
        PrimitiveDataType::DATE => parse_primitive_initial_value!(DATE),
        PrimitiveDataType::TIME_OF_DAY => parse_primitive_initial_value!(TIME_OF_DAY),
        PrimitiveDataType::TOD => parse_primitive_initial_value!(TOD),
        PrimitiveDataType::DATE_AND_TIME => parse_primitive_initial_value!(DATE_AND_TIME),
        PrimitiveDataType::DT => parse_primitive_initial_value!(DT),
    }
}

fn default_initial_value<'a>(
    base_type: &'a BaseType,
    array_size: &'a Option<usize>,
) -> Box<dyn FnOnce() -> Result<InitialValue, ServiceError> + 'a> {
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
            BaseType::Primitive(primitive) => {
                Box::new(move || Ok(InitialValue::Primitive(default_primitive_value(primitive))))
            }
            BaseType::Custom(_) => Box::new(|| Ok(InitialValue::Custom)),
        }
    }
}

fn default_primitive_value(primitive_data_type: &PrimitiveDataType) -> PrimitiveValue {
    match primitive_data_type {
        PrimitiveDataType::BOOL => PrimitiveValue::BOOL(false),
        PrimitiveDataType::SINT => PrimitiveValue::SINT(0),
        PrimitiveDataType::INT => PrimitiveValue::INT(0),
        PrimitiveDataType::DINT => PrimitiveValue::DINT(0),
        PrimitiveDataType::LINT => PrimitiveValue::LINT(0),
        PrimitiveDataType::USINT => PrimitiveValue::USINT(0),
        PrimitiveDataType::UINT => PrimitiveValue::UINT(0),
        PrimitiveDataType::UDINT => PrimitiveValue::UDINT(0),
        PrimitiveDataType::ULINT => PrimitiveValue::ULINT(0),
        PrimitiveDataType::REAL => PrimitiveValue::REAL(0.0),
        PrimitiveDataType::LREAL => PrimitiveValue::LREAL(0.0),
        PrimitiveDataType::BYTE => PrimitiveValue::BYTE(0),
        PrimitiveDataType::WORD => PrimitiveValue::WORD(0),
        PrimitiveDataType::DWORD => PrimitiveValue::DWORD(0),
        PrimitiveDataType::LWORD => PrimitiveValue::LWORD(0),
        PrimitiveDataType::STRING => PrimitiveValue::STRING(String::new()),
        PrimitiveDataType::WSTRING => PrimitiveValue::WSTRING(String::new()),
        PrimitiveDataType::TIME => PrimitiveValue::TIME(0),
        PrimitiveDataType::DATE => PrimitiveValue::DATE(0),
        PrimitiveDataType::TIME_OF_DAY => PrimitiveValue::TIME_OF_DAY(0),
        PrimitiveDataType::TOD => PrimitiveValue::TOD(0),
        PrimitiveDataType::DATE_AND_TIME => PrimitiveValue::DATE_AND_TIME(0),
        PrimitiveDataType::DT => PrimitiveValue::DT(0),
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
