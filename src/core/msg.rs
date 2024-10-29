#[derive(Debug, Clone)]
pub struct StructuredType {
    name: String,
    fields: Vec<Field>,
}

impl StructuredType {
    pub fn new(name: String, fields: Vec<Field>) -> Self {
        Self { name, fields }
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
    name: String,
    base_type: BaseType,
    array_size: Option<ArraySize>,
    field_type: FieldType,
    initial_value: Option<InitialValue>,
    comment: Option<String>,
}

impl Field {
    pub fn new(
        name: String,
        base_type: BaseType,
        array_size: Option<ArraySize>,
        field_type: FieldType,
        initial_value: Option<InitialValue>,
        comment: Option<String>,
    ) -> Self {
        Self {
            name,
            base_type,
            array_size,
            field_type,
            initial_value,
            comment,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn base_type(&self) -> &BaseType {
        &self.base_type
    }
    pub fn array_size(&self) -> Option<&ArraySize> {
        self.array_size.as_ref()
    }
    pub fn field_type(&self) -> &FieldType {
        &self.field_type
    }
    pub fn initial_value(&self) -> Option<&InitialValue> {
        self.initial_value.as_ref()
    }
    pub fn comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }
}

#[derive(Debug, Clone)]
pub enum BaseType {
    Bool,
    Byte,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    Char,
    String(Option<u64>),
    Wstring(Option<u64>),
    Custom(Reference),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Reference {
    Relative { file: String },
    Absolute { package: String, file: String },
}

#[derive(Debug, Clone)]
pub enum ArraySize {
    Capacity(u64),
    Dynamic,
    BoundDynamic(u64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Variable,
    Constant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InitialValue {
    Bool(BoolRepresentation),
    Byte(IntRepresentation),
    Uint8(IntRepresentation),
    Uint16(IntRepresentation),
    Uint32(IntRepresentation),
    Uint64(IntRepresentation),
    Int8(IntRepresentation),
    Int16(IntRepresentation),
    Int32(IntRepresentation),
    Int64(IntRepresentation),
    Float32(f32),
    Float64(f64),
    Char(IntRepresentation),
    String(String),
    Wstring(String),
    Array(Vec<InitialValue>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BoolRepresentation {
    String(bool),
    Binary(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum IntRepresentation {
    SignedDecimal(i64),
    UnsignedDecimal(u64),
    Binary(u64),
    Octal(u64),
    Hexadecimal(u64),
}

pub const ANNOTATION_NAME_IEC61499_WORD: &str = "IEC61499_WORD";
pub const ANNOTATION_NAME_IEC61499_DWORD: &str = "IEC61499_DWORD";
pub const ANNOTATION_NAME_IEC61499_LWORD: &str = "IEC61499_LWORD";
pub const ANNOTATION_NAME_IEC61499_START_INDEX: &str = "IEC61499_StartIndex";
