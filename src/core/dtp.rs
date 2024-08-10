#![allow(non_camel_case_types)]

#[derive(Clone, Debug)]
pub struct DataType {
    name: String,
    comment: Option<String>,
    // identification: Option<Identification>,
    // version_info: Vec<VersionInfo>,
    // compiler_info: Option<CompilerInfo>,
    // asn1_tag: Option<ASN1Tag>,
    data_type_kind: DataTypeKind,
}

impl DataType {
    pub fn new(name: &str, comment: &Option<String>, data_type_kind: &DataTypeKind) -> Self {
        Self {
            name: name.to_string(),
            comment: comment.clone(),
            data_type_kind: data_type_kind.clone(),
        }
    }
    pub fn comment(&self) -> &Option<String> {
        &self.comment
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn data_type_kind(&self) -> &DataTypeKind {
        &self.data_type_kind
    }
}

#[derive(Clone, Debug)]
pub enum DataTypeKind {
    // DirectlyDerivedType(DirectlyDerivedType),
    // EnumeratedType(EnumeratedType),
    // SubrangeType(SubrangeType),
    // ArrayType(ArrayType),
    StructuredType(StructuredType),
}

impl DataTypeKind {
    pub fn matches_any<T: AsRef<str>>(str: T) -> bool {
        match str.as_ref() {
            "StructuredType"
            // | "DirectlyDerivedType"
            // | "EnumeratedType"
            // | "SubrangeType"
            // | "ArrayType"
            => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StructuredType {
    comment: Option<String>,
    children: Vec<StructuredTypeChild>,
}

impl StructuredType {
    pub fn new(comment: &Option<String>, children: &[StructuredTypeChild]) -> Self {
        Self {
            comment: comment.clone(),
            children: children.to_vec(),
        }
    }
    pub fn comment(&self) -> &Option<String> {
        &self.comment
    }
    pub fn children(&self) -> &Vec<StructuredTypeChild> {
        &self.children
    }
}

#[derive(Clone, Debug)]
pub enum StructuredTypeChild {
    VarDeclaration(VarDeclaration),
    // SubrangeVarDeclaration(SubrangeVarDeclaration),
}

impl StructuredTypeChild {
    pub fn matches_any<T: AsRef<str>>(str: T) -> bool {
        match str.as_ref() {
            "VarDeclaration"
            // | "SubrangeVarDeclaration"
            => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct VarDeclaration {
    name: String,
    base_type: BaseType,
    // https://bugs.eclipse.org/bugs/show_bug.cgi?id=581888
    array_size: Option<usize>,
    initial_value: Option<InitialValue>,
    comment: Option<String>,
}

impl VarDeclaration {
    pub fn new(
        name: &str,
        base_type: &BaseType,
        array_size: &Option<usize>,
        initial_value: &Option<InitialValue>,
        comment: &Option<String>,
    ) -> Self {
        Self {
            name: name.to_string(),
            base_type: base_type.clone(),
            array_size: array_size.clone(),
            initial_value: initial_value.clone(),
            comment: comment.clone(),
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn base_type(&self) -> &BaseType {
        &self.base_type
    }
    pub fn array_size(&self) -> &Option<usize> {
        &self.array_size
    }
    pub fn initial_value(&self) -> &Option<InitialValue> {
        &self.initial_value
    }
    pub fn comment(&self) -> &Option<String> {
        &self.comment
    }
}

#[derive(Clone, Debug)]
pub enum BaseType {
    BOOL,
    SINT,
    INT,
    DINT,
    LINT,
    USINT,
    UINT,
    UDINT,
    ULINT,
    REAL,
    LREAL,
    BYTE,
    WORD,
    DWORD,
    LWORD,
    // Der Einfachheit halber wird CHAR von 4diac aufgenommen
    CHAR,
    STRING,
    WSTRING,
    TIME,
    DATE,
    TIME_OF_DAY,
    TOD,
    DATE_AND_TIME,
    DT,
    Custom(String),
}

#[derive(Clone, Debug)]
pub enum InitialValue {
    BOOL(bool),
    SINT(i8),
    INT(i16),
    DINT(i32),
    LINT(i64),
    USINT(u8),
    UINT(u16),
    UDINT(u32),
    ULINT(u64),
    REAL(f32),
    LREAL(f64),
    BYTE(u8),
    WORD(u16),
    DWORD(u32),
    LWORD(u64),
    // Der Einfachheit halber wird CHAR von 4diac aufgenommen
    CHAR(u8),
    STRING(String),
    WSTRING(String),
    TIME(i64),
    DATE(u64),
    TIME_OF_DAY(u64),
    TOD(u64),
    DATE_AND_TIME(u64),
    DT(u64),
    Array(Vec<InitialValue>),
}

pub const XML_TAG_DATA_TYPE: &str = "DataType";
pub const XML_TAG_STRUCTURED_TYPE: &str = "StructuredType";
pub const XML_TAG_VAR_DECLARATION: &str = "VarDeclaration";

pub const XML_ATTRIBUTE_NAME: &str = "Name";
pub const XML_ATTRIBUTE_TYPE: &str = "Type";
pub const XML_ATTRIBUTE_ARRAY_SIZE: &str = "ArraySize";
pub const XML_ATTRIBUTE_INITIAL_VALUE: &str = "InitialValue";
pub const XML_ATTRIBUTE_COMMENT: &str = "Comment";
