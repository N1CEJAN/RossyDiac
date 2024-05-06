#[derive(Debug, Clone)]
pub struct File(Vec<Field>);

impl File {
    pub fn new(field0: Vec<Field>) -> Self {
        Self(field0)
    }
}

#[derive(Debug, Clone)]
pub enum Field {
    Variable(Datatype, Vec<Constraint>, String, Option<String>),
    Constant(Datatype, Vec<Constraint>, String, String),
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

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    BoundedString(usize),
    StaticArray(usize),
    UnboundedDynamicArray,
    BoundedDynamicArray(usize),
}
