#[derive(Debug, Clone)]
pub struct File(Vec<Field>);

impl File {
    pub fn new(field0: Vec<Field>) -> Self {
        Self(field0)
    }
}

#[derive(Debug, Clone)]
pub enum Field {
    Variable(Datatype, Vec<Constraint>, String, Option<Value>),
    Constant(Datatype, Vec<Constraint>, String, Value),
}

#[derive(Debug, Clone)]
pub enum Datatype {
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
    Word,
    Dword,
    Lword,
    Time,
    TimeOfDay,
    Date,
    DateAndTime,
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
pub enum Value {
    Array(Vec<Value>),
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
    Word,
    Dword,
    Lword,
    Time,
    TimeOfDay,
    Date,
    DateAndTime,
}
