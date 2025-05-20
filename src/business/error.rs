#![allow(dead_code)]
use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Custom(String),

    // --Externals
    #[from]
    Io(std::io::Error),
    #[from]
    DtpReader(xmltree::ParseError),
    #[from]
    DtpWriter(xmltree::Error),
    #[from]
    MsgReader(nom::error::Error<String>),
}

// -- Start: Convenience
impl Error {
    pub fn custom(value: impl std::fmt::Display) -> Self {
        Self::Custom(value.to_string())
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Custom(value.to_string())
    }
}
// -- End: Convenience

// -- Start: Boilerplate
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
// -- End: Boilerplate
