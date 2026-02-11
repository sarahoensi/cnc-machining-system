// cutting_data/errors.rs

#[derive(Debug, Clone, PartialEq)]
pub enum DomainError {
    MissingField(&'static str),
    InvalidSpeedMode,
    InvalidFeedMode,
    InvalidValue(&'static str),
}

use std::fmt;

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::MissingField(field) =>
                write!(f, "Missing field: {}", field),

            DomainError::InvalidSpeedMode =>
                write!(f, "Invalid speed mode"),

            DomainError::InvalidFeedMode =>
                write!(f, "Invalid feed mode"),

            DomainError::InvalidValue(msg) =>
                write!(f, "Invalid value: {}", msg),
        }
    }
}

impl std::error::Error for DomainError {}
