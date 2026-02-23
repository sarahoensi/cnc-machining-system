// domain/units/error.rs


use crate::domain::units::{
    angle::AngleError,
    length::LengthUnitError,
    motion::MotionUnitError,
    machining::MachiningUnitError,
};

/// Top-level error for the units subdomain.
///
/// Wraps all specific unit value object errors.
#[derive(Debug)]
pub enum UnitsError {
    Angle(AngleError),
    Length(LengthUnitError),
    Motion(MotionUnitError),
    Machining(MachiningUnitError),
}



//
// Automatic conversions (important for ? operator)
//

impl From<AngleError> for UnitsError {
    fn from(value: AngleError) -> Self {
        UnitsError::Angle(value)
    }
}

impl From<LengthUnitError> for UnitsError {
    fn from(value: LengthUnitError) -> Self {
        UnitsError::Length(value)
    }
}

impl From<MotionUnitError> for UnitsError {
    fn from(value: MotionUnitError) -> Self {
        UnitsError::Motion(value)
    }
}

impl From<MachiningUnitError> for UnitsError {
    fn from(value: MachiningUnitError) -> Self {
        UnitsError::Machining(value)
    }
}

use std::fmt;

impl fmt::Display for UnitsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitsError::Angle(e) => write!(f, "{e}"),
            UnitsError::Length(e) => write!(f, "{e}"),
            UnitsError::Motion(e) => write!(f, "{e}"),
            UnitsError::Machining(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for UnitsError {}