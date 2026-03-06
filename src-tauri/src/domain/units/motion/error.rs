// domain/units/motion/error.rs

use thiserror::Error;

use crate::domain::units::core::NumericError;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum MotionUnitError {
    #[error(transparent)]
    Numeric(#[from] NumericError),
}