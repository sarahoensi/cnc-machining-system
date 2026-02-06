// domain/units/length/radius.rs

use crate::domain::units::errors::UnitError;
use crate::domain::units::length::{Diameter, Length};

/// Radius stored internally as millimeters (mm).
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Radius(f64);

impl Radius {
    pub fn mm(value: f64) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NotFinite("Radius"));
        }
        if value <= 0.0 {
            return Err(UnitError::NonPositiveValue("Radius"));
        }
        Ok(Self(value))
    }

    pub fn mm_value(self) -> f64 {
        self.0
    }

    pub fn as_length(self) -> Length {
        Length::mm(self.0).expect("Radius is always valid length")
    }

    pub fn diameter(self) -> Diameter {
        Diameter::mm(self.0 * 2.0).expect("Radius is > 0 so diameter is > 0")
    }
}
