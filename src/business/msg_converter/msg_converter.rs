use crate::business::error::Result;
use crate::core::dtp;
use crate::core::dtp::{ArraySize, DataType, DataTypeKind, StructuredTypeChild, VarDeclaration};
use crate::core::msg;
use crate::core::msg::{Constraint, Field, FieldType, StructuredType};

pub fn convert(structured_types: Vec<StructuredType>) -> Result<Vec<DataType>> {
    let mut result = Vec::new();
    for structured_type in structured_types.iter() {
        let data_type = convert_structured_type(structured_type)?;
        result.push(data_type);
    }
    Ok(result)
}

fn convert_structured_type(structured_type: &StructuredType) -> Result<DataType> {
    let name = structured_type.name().to_string();
    let mut structured_type_children = Vec::new();
    for field in structured_type.fields().iter() {
        let children = &mut convert_field(field)?;
        structured_type_children.append(children)
    }
    let structured_type = dtp::StructuredType::new(&None, &structured_type_children);
    let data_type_kind = DataTypeKind::StructuredType(structured_type);
    Ok(DataType::new(&name, &None, &data_type_kind))
}

fn convert_field(field: &Field) -> Result<Vec<StructuredTypeChild>> {
    let mut structured_type_children = Vec::new();

    let mut var_name = field.name().to_string();
    let base_type = convert_base_type(field.base_type());

    // handle constraints
    let mut array_size: Option<ArraySize> = None;
    for constraint in field.constraints().iter() {
        match constraint {
            Constraint::StaticArray(capacity) => {
                array_size = Some(ArraySize::Static(*capacity));
            }
            Constraint::UnboundedDynamicArray => {
                array_size = Some(ArraySize::Dynamic);
            }
            Constraint::BoundedDynamicArray(array_bound) => {
                array_size = Some(ArraySize::Dynamic);
                structured_type_children.push(StructuredTypeChild::VarDeclaration(
                    VarDeclaration::new(
                        &format!("{}_array_bound", var_name),
                        &dtp::BaseType::ULINT,
                        &None,
                        &Some(dtp::InitialValue::ULINT(*array_bound as u64)),
                        &None,
                    ),
                ));
            }
            Constraint::BoundedString(string_bound) => {
                structured_type_children.push(StructuredTypeChild::VarDeclaration(
                    VarDeclaration::new(
                        &format!("{}_string_bound", var_name),
                        &dtp::BaseType::ULINT,
                        &None,
                        &Some(dtp::InitialValue::ULINT(*string_bound as u64)),
                        &None,
                    ),
                ));
            }
        }
    }

    // handle initial value
    let optional_initial_value = match field.field_type() {
        FieldType::Variable(optional_initial_value) => {
            convert_optional_initial_value(optional_initial_value, field)
        }
        FieldType::Constant(initial_value) => {
            var_name += "_CONSTANT";
            structured_type_children.clear();
            Some(convert_initial_value(initial_value, field))
        }
    };
    
    structured_type_children.push(StructuredTypeChild::VarDeclaration(VarDeclaration::new(
        &var_name,
        &base_type,
        &array_size,
        &optional_initial_value,
        &None,
    )));
    Ok(structured_type_children)
}

fn convert_base_type(base_type: &msg::BaseType) -> dtp::BaseType {
    match base_type {
        msg::BaseType::Bool => dtp::BaseType::BOOL,
        msg::BaseType::Byte => dtp::BaseType::BYTE,
        msg::BaseType::Float32 => dtp::BaseType::REAL,
        msg::BaseType::Float64 => dtp::BaseType::LREAL,
        msg::BaseType::Int8 => dtp::BaseType::SINT,
        msg::BaseType::Uint8 => dtp::BaseType::USINT,
        msg::BaseType::Int16 => dtp::BaseType::INT,
        msg::BaseType::Uint16 => dtp::BaseType::UINT,
        msg::BaseType::Int32 => dtp::BaseType::DINT,
        msg::BaseType::Uint32 => dtp::BaseType::UDINT,
        msg::BaseType::Int64 => dtp::BaseType::LINT,
        msg::BaseType::Uint64 => dtp::BaseType::ULINT,
        msg::BaseType::String => dtp::BaseType::STRING,
        msg::BaseType::Wstring => dtp::BaseType::WSTRING,
        msg::BaseType::Char => dtp::BaseType::CHAR,
        msg::BaseType::Custom(value) => dtp::BaseType::Custom(value.clone()),
    }
}

fn convert_optional_initial_value(
    optional_initial_value: &Option<msg::InitialValue>,
    context: &Field,
) -> Option<dtp::InitialValue> {
    if let Some(initial_value) = optional_initial_value {
        Some(convert_initial_value(initial_value, context))
    } else {
        None
    }
}

fn convert_initial_value(
    initial_value: &msg::InitialValue,
    context: &Field,
) -> dtp::InitialValue {
    match initial_value {
        msg::InitialValue::Bool(value) => dtp::InitialValue::BOOL(*value),
        msg::InitialValue::Byte(value) => dtp::InitialValue::BYTE(*value),
        msg::InitialValue::Float32(value) => dtp::InitialValue::REAL(*value),
        msg::InitialValue::Float64(value) => dtp::InitialValue::LREAL(*value),
        msg::InitialValue::Int8(value) => dtp::InitialValue::SINT(*value),
        msg::InitialValue::Uint8(value) => dtp::InitialValue::USINT(*value),
        msg::InitialValue::Int16(value) => dtp::InitialValue::INT(*value),
        msg::InitialValue::Uint16(value) => dtp::InitialValue::UINT(*value),
        msg::InitialValue::Int32(value) => dtp::InitialValue::DINT(*value),
        msg::InitialValue::Uint32(value) => dtp::InitialValue::UDINT(*value),
        msg::InitialValue::Int64(value) => dtp::InitialValue::LINT(*value),
        msg::InitialValue::Uint64(value) => dtp::InitialValue::ULINT(*value),
        msg::InitialValue::Char(value) => dtp::InitialValue::CHAR(*value),
        msg::InitialValue::String(value) => dtp::InitialValue::STRING(value.to_string()),
        msg::InitialValue::Wstring(value) => dtp::InitialValue::WSTRING(value.to_string()),
        msg::InitialValue::Array(values) => dtp::InitialValue::Array(
            values
                .iter()
                .map(|value| convert_initial_value(value, context))
                .collect(),
        ),
    }
}
