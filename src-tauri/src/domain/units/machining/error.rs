// domain/units/machining/error.rs

#[derive(Debug, Clone, PartialEq)]
pub enum MachiningUnitError {
    NotFinite { value: f64 },
    NonPositive { value: f64 },
}

use std::fmt;

impl fmt::Display for MachiningUnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MachiningUnitError::NotFinite { value } => {
                write!(f, "Value must be finite, got {value}")
            }

            MachiningUnitError::NonPositive { value } => {
                write!(f, "Value must be greater than 0, got {value}")
            }
        }
    }
}

impl std::error::Error for MachiningUnitError {}