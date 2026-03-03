// domain/units/length/error.rs

use thiserror::Error;
use crate::domain::units::core::NumericError;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum LengthUnitError {
    #[error(transparent)]
    Numeric(#[from] NumericError),
}