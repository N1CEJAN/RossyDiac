#[derive(Debug, Clone)]
pub enum FieldType {
    Primitive {
        datatype: PrimitiveDatatype,
        constraints: Vec<PrimitiveConstraint>,
    },
    Complex {
        datatype: String,
        package: Option<String>,
    },
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
    String,
    Wstring,
}

#[derive(Debug, Clone)]
pub enum PrimitiveConstraint {
    StaticArray(usize),
    UnboundedDynamicArray,
    BoundedDynamicArray(usize),
    BoundedString(usize),
}
