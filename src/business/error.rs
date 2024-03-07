use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ServiceError {
    Io(std::io::Error),
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::Io(error) => write!(f, "{}", error),
        }
    }
}
