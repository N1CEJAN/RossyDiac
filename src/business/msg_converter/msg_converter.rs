use crate::business::error::Result;
use crate::core::dtp;
use crate::core::dtp::{ArraySize, DataType, DataTypeKind, StructuredTypeChild, VarDeclaration};
use crate::core::msg;
use crate::core::msg::{Constraint, Field, FieldType, Reference, StructuredType};

pub fn convert(structured_type: &StructuredType) -> Result<DataType> {
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
    
    // handle BaseType
    let base_type = match field.base_type() {
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
        msg::BaseType::String(constraint) => {
            if let Some(constraint) = constraint {
                structured_type_children.push(StructuredTypeChild::VarDeclaration(
                    VarDeclaration::new(
                        &format!("{}_string_bound", var_name),
                        &dtp::BaseType::ULINT,
                        &None,
                        &Some(dtp::InitialValue::ULINT(*constraint as u64)),
                        &None,
                    ),
                ));
            }
            dtp::BaseType::STRING
        }
        msg::BaseType::Wstring => dtp::BaseType::WSTRING,
        msg::BaseType::Char => dtp::BaseType::CHAR,
        msg::BaseType::Custom(value) => dtp::BaseType::Custom(convert_reference(value)),
    };

    // handle constraints
    let array_size = match field.constraint() {
        Some(Constraint::StaticArray(capacity)) => Some(ArraySize::Static(*capacity)),
        Some(Constraint::UnboundedDynamicArray) => Some(ArraySize::Dynamic),
        Some(Constraint::BoundedDynamicArray(bound)) => {
            structured_type_children.push(StructuredTypeChild::VarDeclaration(
                VarDeclaration::new(
                    &format!("{}_array_bound", var_name),
                    &dtp::BaseType::ULINT,
                    &None,
                    &Some(dtp::InitialValue::ULINT(*bound as u64)),
                    &None,
                ),
            ));
            Some(ArraySize::Dynamic)
        }
        _ => None
    };

    // handle initial value
    // handle constant
    let optional_initial_value = match field.field_type() {
        FieldType::Variable(optional_initial_value) => {
            convert_optional_initial_value(optional_initial_value, field)
        }
        FieldType::Constant(initial_value) => {
            var_name += "_CONSTANT";
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

fn convert_reference(reference: &Reference) -> String {
    match reference {
        Reference::Relative { file } => { file.to_string() }
        Reference::Absolute { package, file } => { format!("{}_{}", package, file) }
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
