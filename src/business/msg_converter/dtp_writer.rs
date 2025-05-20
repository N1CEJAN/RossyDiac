use std::borrow::Cow;
use std::fs::File;
use std::string::ToString;

use xmltree::{Element, EmitterConfig, XMLNode};

use crate::business::error::Result;
use crate::core::dtp::*;

pub fn write(data_type: DataType, to_directory: &str) -> Result<()> {
    let data_type_name = data_type.name().to_string();
    let custom_data_type_element = create_data_type_element(data_type);

    let mut config = EmitterConfig::new();
    config.perform_escaping = false;
    config.perform_indent = true;
    config.indent_string = Cow::Borrowed("    ");
    config.pad_self_closing = false;
    let file = File::create(format!("{to_directory}{data_type_name}.dtp"))?;
    custom_data_type_element.write_with_config(file, config)?;
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
        .push(create_structured_type_element(data_type.structured_type()));
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
            .var_declarations()
            .iter()
            .map(create_var_declaration_element)
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
            match array_size {
                ArraySize::Capacity(capacity) => format!("{capacity}"),
                ArraySize::Indexation(start, end) => format!("{start}..{end}"),
            },
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
    var_declaration_element.children.append(
        &mut var_declaration
            .attributes()
            .iter()
            .map(create_attribute_element)
            .collect(),
    );
    XMLNode::Element(var_declaration_element)
}

fn create_attribute_element(attribute: &Attribute) -> XMLNode {
    let mut attribute_element = Element::new(XML_TAG_ATTRIBUTE);
    attribute_element
        .attributes
        .insert(XML_ATTRIBUTE_NAME.to_owned(), attribute.name().to_owned());
    attribute_element.attributes.insert(
        XML_ATTRIBUTE_TYPE.to_owned(),
        base_type_to_string(attribute.base_type()),
    );
    attribute_element.attributes.insert(
        XML_ATTRIBUTE_VALUE.to_owned(),
        initial_value_to_string(attribute.value()),
    );
    if let Some(comment) = attribute.comment() {
        attribute_element
            .attributes
            .insert(XML_ATTRIBUTE_COMMENT.to_owned(), comment.to_owned());
    }
    XMLNode::Element(attribute_element)
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
        BaseType::STRING(opt_bound) => opt_bound
            .map(|bound| format!("STRING[{bound}]"))
            .unwrap_or_else(|| "STRING".to_string()),
        BaseType::WSTRING(opt_bound) => opt_bound
            .map(|bound| format!("WSTRING[{bound}]"))
            .unwrap_or_else(|| "WSTRING".to_string()),
        BaseType::Custom(type_name) => type_name.clone(),
    }
}

fn initial_value_to_string(initial_value: &InitialValue) -> String {
    match initial_value {
        InitialValue::BOOL(bool_representation) => {
            bool_representation_as_string(bool_representation)
        }
        InitialValue::BYTE(int_representation)
        | InitialValue::WORD(int_representation)
        | InitialValue::DWORD(int_representation)
        | InitialValue::LWORD(int_representation)
        | InitialValue::USINT(int_representation)
        | InitialValue::UINT(int_representation)
        | InitialValue::UDINT(int_representation)
        | InitialValue::ULINT(int_representation)
        | InitialValue::SINT(int_representation)
        | InitialValue::INT(int_representation)
        | InitialValue::DINT(int_representation)
        | InitialValue::LINT(int_representation) => {
            int_representation_as_string(int_representation)
        }
        InitialValue::REAL(real_representation) => real_representation.to_string(),
        InitialValue::LREAL(lreal_representation) => lreal_representation.to_string(),
        InitialValue::CHAR(char_representation) => {
            char_representation_as_string(char_representation)
        }
        InitialValue::STRING(string_representation) => {
            string_representation_as_string(string_representation)
        }
        InitialValue::WSTRING(wstring_representation) => {
            wstring_representation_as_string(wstring_representation)
        }
        InitialValue::Array(v) => format!(
            "[{}]",
            v.iter()
                .map(initial_value_to_string)
                .collect::<Vec<String>>()
                .join(", ")
        ),
    }
}

fn bool_representation_as_string(bool_representation: &BoolRepresentation) -> String {
    match bool_representation {
        BoolRepresentation::String(true) => "TRUE".to_string(),
        BoolRepresentation::String(false) => "FALSE".to_string(),
        BoolRepresentation::Binary(true) => "1".to_string(),
        BoolRepresentation::Binary(false) => "0".to_string(),
    }
}

fn int_representation_as_string(int_representation: &IntRepresentation) -> String {
    match int_representation {
        IntRepresentation::SignedDecimal(i64) => format!("{i64}"),
        IntRepresentation::UnsignedDecimal(u64) => format!("{u64}"),
        IntRepresentation::Binary(u64) => format!("2#{u64:b}"),
        IntRepresentation::Octal(u64) => format!("8#{u64:o}"),
        IntRepresentation::Heaxdecimal(u64) => format!("16#{u64:X}"),
    }
}

fn char_representation_as_string(char_representation: &CharRepresentation) -> String {
    let literal = format!("'{}'", char_reprensentation_as_string(char_representation));
    mask_html_special_character(literal)
}

fn string_representation_as_string(string_representation: &[CharRepresentation]) -> String {
    let literal = format!(
        "'{}'",
        string_representation
            .iter()
            .map(char_reprensentation_as_string)
            .collect::<Vec<_>>()
            .join("")
    );
    mask_html_special_character(literal)
}

fn wstring_representation_as_string(wstring_representation: &[WcharRepresentation]) -> String {
    let literal = format!(
        "\"{}\"",
        wstring_representation
            .iter()
            .map(wchar_representation_as_string)
            .collect::<Vec<_>>()
            .join("")
    );
    mask_html_special_character(literal)
}

fn char_reprensentation_as_string(char_representation: &CharRepresentation) -> String {
    match char_representation {
        CharRepresentation::Char(char) if matches!(char, '$' | '\'') => format!("${char}"),
        CharRepresentation::Char(char) => char.to_string(),
        CharRepresentation::Hexadecimal(char) => format!("${:02X}", *char as u8),
    }
}

fn wchar_representation_as_string(wchar_representation: &WcharRepresentation) -> String {
    match wchar_representation {
        WcharRepresentation::Wchar(wchar) if matches!(wchar, '$' | '"') => format!("${wchar}"),
        WcharRepresentation::Wchar(wchar) => wchar.to_string(),
        WcharRepresentation::Hexadecimal(wchar) => format!("${:04X}", *wchar as u32),
    }
}

fn mask_html_special_character(string: String) -> String {
    let mut masked = String::with_capacity(string.len());
    for c in string.chars() {
        match c {
            '&' => masked.push_str("&amp;"),
            '<' => masked.push_str("&lt;"),
            '>' => masked.push_str("&gt;"),
            '"' => masked.push_str("&quot;"),
            '\'' => masked.push_str("&apos;"),
            // ' ' => masked.push_str("&nbsp;"),
            _ => masked.push(c),
        }
    }
    masked
}
