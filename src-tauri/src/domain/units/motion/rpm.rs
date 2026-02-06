// domain/units/motion/rpm.rs

use crate::domain::units::errors::UnitError;

/// Revolutions per minute (RPM).
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Rpm(f64);

impl Rpm {
    pub fn new(value: f64) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NotFinite("Rpm"));
        }
        if value <= 0.0 {
            return Err(UnitError::NonPositiveValue("Rpm"));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> f64 {
        self.0
    }
}
