// domain/machining/cutting_data/error.rs

use crate::domain::units::{MachiningUnitError, MotionUnitError, UnitsError};
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum CuttingError {
    #[error(transparent)]
    Unit(#[from] UnitsError),

    #[error(transparent)]
    Motion(#[from] MotionUnitError),

    #[error(transparent)]
    Machining(#[from] MachiningUnitError),

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Numerical instability detected")]
    NumericalInstability,
}
