#[derive(Clone, Debug)]
pub struct CustomDataType {
    name: String,
    comment: String,
    // identification: Option<Identification>,
    // version_info: Vec<VersionInfo>,
    // compiler_info: Option<CompilerInfo>,
    // asn1_tag: Option<ASN1Tag>,
    data_type_kind: DataTypeKind,
}

impl CustomDataType {
    pub fn new(name: &str, comment: &str, data_type_kind: &DataTypeKind) -> Self {
        Self {
            name: name.to_string(),
            comment: comment.to_string(),
            data_type_kind: data_type_kind.clone(),
        }
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
    comment: String,
    children: Vec<StructuredTypeChild>,
}

impl StructuredType {
    pub fn new(comment: &str, children: &Vec<StructuredTypeChild>) -> Self {
        Self {
            comment: comment.to_string(),
            children: children.to_vec(),
        }
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
    array_size: Option<usize>,
    initial_value: InitialValue,
    comment: String,
}

impl VarDeclaration {
    pub fn new(
        name: &str,
        base_type: &BaseType,
        array_size: &Option<usize>,
        initial_value: &InitialValue,
        comment: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            base_type: base_type.clone(),
            array_size: array_size.clone(),
            initial_value: initial_value.clone(),
            comment: comment.to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum BaseType {
    Primitive(PrimitiveDataType),
    Custom(String),
}

#[derive(Clone, Debug)]
pub enum PrimitiveDataType {
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
    STRING,
    WSTRING,
    TIME,
    DATE,
    TIME_OF_DAY,
    TOD,
    DATE_AND_TIME,
    DT,
}

#[derive(Clone, Debug)]
pub enum InitialValue {
    Primitive(PrimitiveValue),
    Custom,
    Array(Vec<InitialValue>),
}

#[derive(Clone, Debug)]
pub enum PrimitiveValue {
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
    STRING(String),
    WSTRING(String),
    TIME(i64),
    DATE(u64),
    TIME_OF_DAY(u64),
    TOD(u64),
    DATE_AND_TIME(u64),
    DT(u64),
}
