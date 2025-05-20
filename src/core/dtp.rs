#![allow(non_camel_case_types)]

#[derive(Clone, Debug)]
pub struct DataType {
    name: String,
    comment: Option<String>,
    structured_type: StructuredType,
}

impl DataType {
    pub fn new(name: String, comment: Option<String>, structured_type: StructuredType) -> Self {
        Self {
            name,
            comment,
            structured_type,
        }
    }
    pub fn comment(&self) -> &Option<String> {
        &self.comment
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn structured_type(&self) -> &StructuredType {
        &self.structured_type
    }
}

#[derive(Clone, Debug)]
pub struct StructuredType {
    comment: Option<String>,
    var_declarations: Vec<VarDeclaration>,
}

impl StructuredType {
    pub fn new(comment: Option<String>, children: Vec<VarDeclaration>) -> Self {
        Self {
            comment,
            var_declarations: children,
        }
    }
    pub fn comment(&self) -> &Option<String> {
        &self.comment
    }
    pub fn var_declarations(&self) -> &Vec<VarDeclaration> {
        &self.var_declarations
    }
}

#[derive(Clone, Debug)]
pub struct VarDeclaration {
    name: String,
    base_type: BaseType,
    array_size: Option<ArraySize>,
    initial_value: Option<InitialValue>,
    comment: Option<String>,
    attributes: Vec<Attribute>,
}

impl VarDeclaration {
    pub fn new(
        name: String,
        base_type: BaseType,
        array_size: Option<ArraySize>,
        initial_value: Option<InitialValue>,
        comment: Option<String>,
        attributes: Vec<Attribute>,
    ) -> Self {
        Self {
            name,
            base_type,
            array_size,
            initial_value,
            comment,
            attributes,
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
    pub fn initial_value(&self) -> Option<&InitialValue> {
        self.initial_value.as_ref()
    }
    pub fn comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }
    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }
}

#[derive(Clone, Debug)]
pub struct Attribute {
    name: String,
    base_type: BaseType,
    value: InitialValue,
    comment: Option<String>,
}

impl Attribute {
    pub fn new(
        name: String,
        base_type: BaseType,
        value: InitialValue,
        comment: Option<String>,
    ) -> Self {
        Self {
            name,
            base_type,
            value,
            comment,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn base_type(&self) -> &BaseType {
        &self.base_type
    }
    pub fn value(&self) -> &InitialValue {
        &self.value
    }
    pub fn comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, PartialEq)]
pub enum BaseType {
    BOOL,
    BYTE,
    WORD,
    DWORD,
    LWORD,
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
    CHAR,
    STRING(Option<u64>),
    WSTRING(Option<u64>),
    Custom(String),
}

#[derive(Clone, Debug)]
pub enum ArraySize {
    Capacity(u64),
    Indexation(i64, i64),
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, PartialEq)]
pub enum InitialValue {
    BOOL(BoolRepresentation),
    BYTE(IntRepresentation),
    WORD(IntRepresentation),
    DWORD(IntRepresentation),
    LWORD(IntRepresentation),
    USINT(IntRepresentation),
    UINT(IntRepresentation),
    UDINT(IntRepresentation),
    ULINT(IntRepresentation),
    SINT(IntRepresentation),
    INT(IntRepresentation),
    DINT(IntRepresentation),
    LINT(IntRepresentation),
    REAL(f32),
    LREAL(f64),
    CHAR(CharRepresentation),
    STRING(Vec<CharRepresentation>),
    WSTRING(Vec<WcharRepresentation>),
    Array(Vec<InitialValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BoolRepresentation {
    String(bool),
    Binary(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum IntRepresentation {
    SignedDecimal(i64),
    UnsignedDecimal(u64),
    Binary(u64),
    Octal(u64),
    Heaxdecimal(u64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CharRepresentation {
    Char(char),
    Hexadecimal(char),
}

#[derive(Clone, Debug, PartialEq)]
pub enum WcharRepresentation {
    Wchar(char),
    Hexadecimal(char),
}

pub const XML_TAG_DATA_TYPE: &str = "DataType";
pub const XML_TAG_STRUCTURED_TYPE: &str = "StructuredType";
pub const XML_TAG_VAR_DECLARATION: &str = "VarDeclaration";
pub const XML_TAG_ATTRIBUTE: &str = "Attribute";
pub const XML_ATTRIBUTE_NAME: &str = "Name";
pub const XML_ATTRIBUTE_TYPE: &str = "Type";
pub const XML_ATTRIBUTE_ARRAY_SIZE: &str = "ArraySize";
pub const XML_ATTRIBUTE_INITIAL_VALUE: &str = "InitialValue";
pub const XML_ATTRIBUTE_VALUE: &str = "Value";
pub const XML_ATTRIBUTE_COMMENT: &str = "Comment";
pub const ANNOTATION_NAME_ROS2_RELATIVE_REFERENCE: &str = "ROS2_RelativeReference";
pub const ANNOTATION_NAME_ROS2_ABSOLUTE_REFERENCE: &str = "ROS2_AbsoluteReference";
pub const ANNOTATION_NAME_ROS2_DYNAMIC_ARRAY: &str = "ROS2_DynamicArray";
pub const ANNOTATION_NAME_ROS2_BOUND_DYNAMIC_ARRAY: &str = "ROS2_BoundDynamicArray";
pub const ANNOTATION_NAME_ROS2_ELEMENT_COUNTER: &str = "ROS2_ElementCounter";
pub const ANNOTATION_NAME_ROS2_CONSTANT: &str = "ROS2_Constant";
