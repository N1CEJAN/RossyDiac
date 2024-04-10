use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ServiceError {
    Io(String),
    Parser(String),
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::Io(msg) => write!(f, "{}", msg),
            ServiceError::Parser(msg) => write!(f, "{}", msg),
        }
    }
}
