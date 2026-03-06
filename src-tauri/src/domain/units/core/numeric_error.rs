// domain/units/core/numeric_error.rs

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum NumericError {
    #[error("Value must be finite, got {0}")]
    NotFinite(f64),

    #[error("Value must be greater than 0, got {0}")]
    NonPositive(f64),

    #[error("Value must not be negative, got {0}")]
    Negative(f64),
}