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
}

#[derive(Debug, Clone)]
pub enum Field {
    Variable(BaseType, Vec<Constraint>, String, Option<InitialValue>),
    Constant(BaseType, Vec<Constraint>, String, InitialValue),
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
    String,
    Wstring,
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum Constraint {
    BoundedString(usize),
    StaticArray(usize),
    UnboundedDynamicArray,
    BoundedDynamicArray(usize),
}

#[derive(Debug, Clone)]
pub enum InitialValue {
    Bool(bool),
    Byte(u8),
    Float32(f32),
    Float64(f64),
    Int8(i8),
    Uint8(u8),
    Int16(i16),
    Uint16(u16),
    Int32(i32),
    Uint32(u32),
    Int64(i64),
    Uint64(u64),
    Char(char),
    String(String),
    Wstring(String),
    Custom,
    Array(Vec<InitialValue>),
}
