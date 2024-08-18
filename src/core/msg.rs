#[derive(Debug, Clone)]
pub struct StructuredType {
    name: String,
    fields: Vec<Field>,
}

impl StructuredType {
    pub fn new(name: &str, fields: Vec<Field>) -> Self {
        Self {
            name: name.to_string().clone(),
            fields,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    base_type: BaseType,
    constraint: Option<Constraint>,
    name: String,
    field_type: FieldType,
}

impl Field {
    pub fn new(
        base_type: &BaseType,
        constraint: &Option<Constraint>,
        name: &str,
        field_type: &FieldType,
    ) -> Self {
        Self {
            base_type: base_type.clone(),
            constraint: constraint.clone(),
            name: name.to_string(),
            field_type: field_type.clone(),
        }
    }
    pub fn base_type(&self) -> &BaseType {
        &self.base_type
    }
    pub fn constraint(&self) -> Option<&Constraint> {
        self.constraint.as_ref()
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn field_type(&self) -> &FieldType {
        &self.field_type
    }
}

#[derive(Debug, Clone)]
pub enum BaseType {
    Bool,
    Byte,
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
    Char,
    String(Option<usize>),
    Wstring(Option<usize>),
    Custom(Reference),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Reference {
    Relative { file: String },
    Absolute { package: String, file: String },
}

#[derive(Debug, Clone)]
pub enum Constraint {
    StaticArray(usize),
    UnboundedDynamicArray,
    BoundedDynamicArray(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    // http://design.ros2.org/articles/generated_interfaces_cpp.html#constructors
    // Auflistung: MessageInitialization::ALL
    // Der Default der C++ Code Generierung generiert immer ein
    // InitialValue, jedoch gibt es auch einen Opt-Out.
    Variable(Option<InitialValue>),
    Constant(InitialValue),
}

#[derive(Debug, Clone, PartialEq)]
pub enum InitialValue {
    Bool(bool),
    Byte(u8),
    Float32(f32),
    Float64(f64),
    Int8(IntLiteral),
    Uint8(u8),
    Int16(i16),
    Uint16(u16),
    Int32(i32),
    Uint32(u32),
    Int64(i64),
    Uint64(u64),
    // http://design.ros2.org/articles/idl_interface_definition.html
    // A 8-bit single-byte character with a numerical value
    // between 0 and 255 (see 7.2.6.2.1)
    // http://design.ros2.org/articles/generated_interfaces_cpp.html#constructors
    // Constructors: [...](note: char fields are considered numeric for C++).
    Char(u8),
    String(String),
    Wstring(String),
    Array(Vec<InitialValue>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntLiteral {
    pub value: String,
    pub e_int_literal: EIntLiteral,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EIntLiteral {
    DecimalInt,
    BinaryInt,
    OctalInt,
    HexalInt,
}
