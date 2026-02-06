// units/angle/angle.rs

use crate::domain::units::errors::UnitError;

/// Angle stored internally as radians.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Angle(f64);

impl Angle {
    pub fn radians(value: f64) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NotFinite("Angle"));
        }
        Ok(Self(value))
    }

    pub fn degrees(value: f64) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NotFinite("Angle"));
        }
        Ok(Self(value.to_radians()))
    }

    pub fn radians_value(self) -> f64 {
        self.0
    }

    pub fn degrees_value(self) -> f64 {
        self.0.to_degrees()
    }
}
