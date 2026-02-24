// domain/units/length/error.rs

#[derive(Debug, Clone, PartialEq)]
pub enum LengthUnitError {
    NotFinite {
        value: f64,
    },

    Negative {
        value: f64,
    },

    NonPositive {
        value: f64,
    },
}

use std::fmt;

impl fmt::Display for LengthUnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LengthUnitError::NotFinite { value } => {
                write!(f, "Length must be finite, got {value}")
            }

            LengthUnitError::Negative { value } => {
                write!(f, "Length must not be negative, got {value}")
            }

            LengthUnitError::NonPositive { value } => {
                write!(f, "Length must be greater than 0, got {value}")
            }
        }
    }
}

impl std::error::Error for LengthUnitError {}