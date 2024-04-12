#[derive(Debug, Clone)]
pub struct FieldType {
    datatype: Datatype,
    constraint: Vec<Constraint>,
}

impl FieldType {
    pub fn new(datatype: Datatype, constraint: Vec<Constraint>) -> Self {
        Self {
            datatype,
            constraint,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Datatype {
    Primitive(PrimitiveDatatype),
    String(StringDatatype),
    Complex(Option<String>, String),
}

#[derive(Debug, Clone)]
pub enum PrimitiveDatatype {
    Bool,
    Byte,
    Char,
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
}

#[derive(Debug, Clone)]
pub enum StringDatatype {
    String,
    Wstring,
}

#[derive(Debug, Clone)]
pub enum Constraint {
    StringConstraint(StringConstraint),
    ArrayConstraint(ArrayConstraint),
}

#[derive(Debug, Clone)]
pub enum StringConstraint {
    BoundedString(usize),
}

#[derive(Debug, Clone)]
pub enum ArrayConstraint {
    StaticArray(usize),
    UnboundedDynamicArray,
    BoundedDynamicArray(usize),
}
