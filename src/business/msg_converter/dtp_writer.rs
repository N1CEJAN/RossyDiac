use std::fs::File;
use std::string::ToString;

use xmltree::{Element, EmitterConfig, XMLNode};

use crate::business::error::Result;
use crate::core::dtp::*;

pub fn write(data_types: Vec<DataType>, to_directory: &str) -> Result<()> {
    for data_type in data_types.into_iter() {
        let data_type_name = data_type.name().to_string();
        let custom_data_type_element = create_data_type_element(data_type);

        let mut config = EmitterConfig::new();
        config.perform_indent = true;
        let file = File::create(format!("{to_directory}{data_type_name}.dtp"))?;
        custom_data_type_element.write_with_config(file, config)?;
    }
    Ok(())
}

fn create_data_type_element(data_type: DataType) -> Element {
    let mut data_type_element = Element::new(XML_TAG_DATA_TYPE);
    data_type_element
        .attributes
        .insert(XML_ATTRIBUTE_NAME.to_string(), data_type.name().to_string());
    if let Some(comment) = data_type.comment() {
        data_type_element
            .attributes
            .insert(XML_ATTRIBUTE_COMMENT.to_string(), comment.clone());
    }
    data_type_element
        .children
        .push(match data_type.data_type_kind() {
            DataTypeKind::StructuredType(structured_type) => {
                create_structured_type_element(structured_type)
            }
        });
    data_type_element
}

fn create_structured_type_element(structured_type: &StructuredType) -> XMLNode {
    let mut structured_type_element = Element::new(XML_TAG_STRUCTURED_TYPE);
    if let Some(comment) = structured_type.comment() {
        structured_type_element
            .attributes
            .insert(XML_ATTRIBUTE_COMMENT.to_string(), comment.clone());
    }
    structured_type_element.children.append(
        &mut structured_type
            .children()
            .iter()
            .map(|structured_type_child| match structured_type_child {
                StructuredTypeChild::VarDeclaration(var_declaration) => {
                    create_var_declaration_element(var_declaration)
                }
            })
            .collect(),
    );
    XMLNode::Element(structured_type_element)
}

fn create_var_declaration_element(var_declaration: &VarDeclaration) -> XMLNode {
    let mut var_declaration_element = Element::new(XML_TAG_VAR_DECLARATION);
    var_declaration_element.attributes.insert(
        XML_ATTRIBUTE_NAME.to_string(),
        var_declaration.name().to_string(),
    );
    var_declaration_element.attributes.insert(
        XML_ATTRIBUTE_TYPE.to_string(),
        base_type_to_string(var_declaration.base_type()),
    );
    if let Some(array_size) = var_declaration.array_size() {
        var_declaration_element.attributes.insert(
            XML_ATTRIBUTE_ARRAY_SIZE.to_string(),
            array_size.to_string(),
        );
    }
    if let Some(initial_value) = var_declaration.initial_value() {
        var_declaration_element.attributes.insert(
            XML_ATTRIBUTE_INITIAL_VALUE.to_string(),
            initial_value_to_string(initial_value),
        );
    }
    if let Some(comment) = var_declaration.comment() {
        var_declaration_element
            .attributes
            .insert(XML_ATTRIBUTE_COMMENT.to_string(), comment.clone());
    }
    XMLNode::Element(var_declaration_element)
}

fn base_type_to_string(base_type: &BaseType) -> String {
    match base_type {
        BaseType::BOOL => "BOOL".to_string(),
        BaseType::SINT => "SINT".to_string(),
        BaseType::INT => "INT".to_string(),
        BaseType::DINT => "DINT".to_string(),
        BaseType::LINT => "LINT".to_string(),
        BaseType::USINT => "USINT".to_string(),
        BaseType::UINT => "UINT".to_string(),
        BaseType::UDINT => "UDINT".to_string(),
        BaseType::ULINT => "ULINT".to_string(),
        BaseType::REAL => "REAL".to_string(),
        BaseType::LREAL => "LREAL".to_string(),
        BaseType::BYTE => "BYTE".to_string(),
        BaseType::WORD => "WORD".to_string(),
        BaseType::DWORD => "DWORD".to_string(),
        BaseType::LWORD => "LWORD".to_string(),
        BaseType::CHAR => "CHAR".to_string(),
        BaseType::STRING => "STRING".to_string(),
        BaseType::WSTRING => "WSTRING".to_string(),
        BaseType::TIME => "TIME".to_string(),
        BaseType::DATE => "DATE".to_string(),
        BaseType::TIME_OF_DAY => "TIME_OF_DAY".to_string(),
        BaseType::TOD => "TOD".to_string(),
        BaseType::DATE_AND_TIME => "DATE_AND_TIME".to_string(),
        BaseType::DT => "DT".to_string(),
        BaseType::Custom(type_name) => type_name.clone(),
    }
}

fn initial_value_to_string(initial_value: &InitialValue) -> String {
    match initial_value {
        InitialValue::BOOL(bool) => {
            if *bool {
                "TRUE".to_string()
            } else {
                "FALSE".to_string()
            }
        }
        InitialValue::SINT(value) => value.to_string(),
        InitialValue::INT(value) => value.to_string(),
        InitialValue::DINT(value) => value.to_string(),
        InitialValue::LINT(value) => value.to_string(),
        InitialValue::USINT(value) => value.to_string(),
        InitialValue::UINT(value) => value.to_string(),
        InitialValue::UDINT(value) => value.to_string(),
        InitialValue::ULINT(value) => value.to_string(),
        InitialValue::REAL(value) => value.to_string(),
        InitialValue::LREAL(value) => value.to_string(),
        InitialValue::BYTE(value) => value.to_string(),
        InitialValue::WORD(value) => value.to_string(),
        InitialValue::DWORD(value) => value.to_string(),
        InitialValue::LWORD(value) => value.to_string(),
        InitialValue::CHAR(value) => value.to_string(),
        InitialValue::STRING(value) => value.to_string(), // TODO: Checken
        InitialValue::WSTRING(value) => value.to_string(), // TODO: Checken
        InitialValue::TIME(value) => value.to_string(),
        InitialValue::DATE(value) => value.to_string(),
        InitialValue::TIME_OF_DAY(value) => value.to_string(),
        InitialValue::TOD(value) => value.to_string(),
        InitialValue::DATE_AND_TIME(value) => value.to_string(),
        InitialValue::DT(value) => value.to_string(),
        InitialValue::Array(values) => array_of_initial_values_as_string(values),
    }
}

fn array_of_initial_values_as_string(values: &[InitialValue]) -> String {
    format!(
        "{{{}}}",
        values
            .iter()
            .map(initial_value_to_string)
            .collect::<Vec<String>>()
            .join(",")
    )
}

// fn default_initial_value<'a>(
//     base_type: &'a BaseType,
//     array_size: &'a Option<usize>,
// ) -> Box<dyn FnOnce() -> Result<InitialValue> + 'a> {
//     if let Some(array_capacity) = array_size {
//         Box::new(move || {
//             let mut initial_values = Vec::with_capacity(*array_capacity);
//             for _ in 0..*array_capacity {
//                 initial_values.push(default_initial_value(base_type, &None)()?)
//             }
//             Ok(InitialValue::Array(initial_values))
//         })
//     } else {
//         match base_type {
//             BaseType::BOOL => Box::new(|| Ok(InitialValue::BOOL(false))),
//             BaseType::SINT => Box::new(|| Ok(InitialValue::SINT(0))),
//             BaseType::INT => Box::new(|| Ok(InitialValue::INT(0))),
//             BaseType::DINT => Box::new(|| Ok(InitialValue::DINT(0))),
//             BaseType::LINT => Box::new(|| Ok(InitialValue::LINT(0))),
//             BaseType::USINT => Box::new(|| Ok(InitialValue::USINT(0))),
//             BaseType::UINT => Box::new(|| Ok(InitialValue::UINT(0))),
//             BaseType::UDINT => Box::new(|| Ok(InitialValue::UDINT(0))),
//             BaseType::ULINT => Box::new(|| Ok(InitialValue::ULINT(0))),
//             BaseType::REAL => Box::new(|| Ok(InitialValue::REAL(0.0))),
//             BaseType::LREAL => Box::new(|| Ok(InitialValue::LREAL(0.0))),
//             BaseType::BYTE => Box::new(|| Ok(InitialValue::BYTE(0))),
//             BaseType::WORD => Box::new(|| Ok(InitialValue::WORD(0))),
//             BaseType::DWORD => Box::new(|| Ok(InitialValue::DWORD(0))),
//             BaseType::LWORD => Box::new(|| Ok(InitialValue::LWORD(0))),
//             BaseType::STRING => Box::new(|| Ok(InitialValue::STRING(String::new()))),
//             BaseType::WSTRING => Box::new(|| Ok(InitialValue::WSTRING(String::new()))),
//             BaseType::TIME => Box::new(|| Ok(InitialValue::TIME(0))),
//             BaseType::DATE => Box::new(|| Ok(InitialValue::DATE(0))),
//             BaseType::TIME_OF_DAY => Box::new(|| Ok(InitialValue::TIME_OF_DAY(0))),
//             BaseType::TOD => Box::new(|| Ok(InitialValue::TOD(0))),
//             BaseType::DATE_AND_TIME => Box::new(|| Ok(InitialValue::DATE_AND_TIME(0))),
//             BaseType::DT => Box::new(|| Ok(InitialValue::DT(0))),
//             BaseType::Custom(_) => Box::new(|| Ok(InitialValue::Custom)),
//         }
//     }
// }
