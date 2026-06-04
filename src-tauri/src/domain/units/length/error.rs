// domain/units/length/error.rs

use crate::domain::units::core::NumericError;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum LengthUnitError {
    #[error(transparent)]
    Numeric(#[from] NumericError),
}
