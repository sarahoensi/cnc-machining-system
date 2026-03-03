// domain/units/machining/error.rs

use thiserror::Error;

use crate::domain::units::core::NumericError;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum MachiningUnitError {
    #[error(transparent)]
    Numeric(#[from] NumericError),
}