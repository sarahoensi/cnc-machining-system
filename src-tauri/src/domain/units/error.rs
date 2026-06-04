// domain/units/error.rs

use thiserror::Error;

use crate::domain::units::{
    angle::AngleError, core::NumericError, length::LengthUnitError, machining::MachiningUnitError,
    motion::MotionUnitError,
};

/// Top-level error for the units subdomain.
///
/// Wraps all specific unit value object errors.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum UnitsError {
    #[error(transparent)]
    Numeric(#[from] NumericError),

    #[error(transparent)]
    Angle(#[from] AngleError),

    #[error(transparent)]
    Length(#[from] LengthUnitError),

    #[error(transparent)]
    Motion(#[from] MotionUnitError),

    #[error(transparent)]
    Machining(#[from] MachiningUnitError),
}
