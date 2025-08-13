use std::fmt;

#[derive(Debug)]
pub struct DomainError(pub String);

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DomainError: {}", self.0)
    }
}

impl std::error::Error for DomainError {}
