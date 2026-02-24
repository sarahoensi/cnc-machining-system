// domain/units/motion/error.rs

#[derive(Debug, Clone, PartialEq)]
pub enum MotionUnitError {
    NotFinite { value: f64 },
    NonPositive { value: f64 },
}

use std::fmt;

impl fmt::Display for MotionUnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MotionUnitError::NotFinite { value } => {
                write!(f, "Value must be finite, got {value}")
            }

            MotionUnitError::NonPositive { value } => {
                write!(f, "Value must be greater than 0, got {value}")
            }
        }
    }
}

impl std::error::Error for MotionUnitError {}