// domain/units/length/error.rs

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum LengthUnitError {
    #[error("Length must be finite, got {value}")]
    NotFinite {
        value: f64,
    },

    #[error("Length must not be negative, got {value}")]
    Negative {
        value: f64,
    },

    #[error("Length must be greater than 0, got {value}")]
    NonPositive {
        value: f64,
    },
}