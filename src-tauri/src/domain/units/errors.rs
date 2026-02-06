// domain/units/error.rs

use std::fmt;

/// Common validation errors for unit value objects.
#[derive(Debug, Clone, PartialEq)]
pub enum UnitError {
    NotFinite(&'static str),
    NonPositiveValue(&'static str),
    NegativeValue(&'static str),
    OutOfRange {
        ty: &'static str,
        min: f64,
        max: f64,
        actual: f64,
    },
}

impl fmt::Display for UnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitError::NotFinite(ty) => write!(f, "{ty} must be finite"),
            UnitError::NonPositiveValue(ty) => write!(f, "{ty} must be > 0"),
            UnitError::NegativeValue(ty) => write!(f, "{ty} must be >= 0"),
            UnitError::OutOfRange { ty, min, max, actual } => {
                write!(f, "{ty} out of range [{min}, {max}]: {actual}")
            }
        }
    }
}

impl std::error::Error for UnitError {}
