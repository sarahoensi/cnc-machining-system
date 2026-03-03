// domain/units/machining/error.rs

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum MachiningUnitError {
    #[error("Value must be finite, got {0}")]
    NotFinite(f64),

    #[error("Value must be greater than 0, got {0}")]
    NonPositive(f64),
}