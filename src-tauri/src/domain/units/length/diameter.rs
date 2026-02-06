// domain/units/length/diameter.rs

use crate::domain::units::errors::UnitError;
use crate::domain::units::length::{Length, Radius};

/// Diameter stored internally as millimeters (mm).
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Diameter(f64);

impl Diameter {
    pub fn mm(value: f64) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NotFinite("Diameter"));
        }
        if value <= 0.0 {
            return Err(UnitError::NonPositiveValue("Diameter"));
        }
        Ok(Self(value))
    }

    pub fn mm_value(self) -> f64 {
        self.0
    }

    pub fn as_length(self) -> Length {
        // safe because diameter is always finite and > 0
        Length::mm(self.0).expect("Diameter is always valid length")
    }

    pub fn radius(self) -> Radius {
        Radius::mm(self.0 / 2.0).expect("Diameter is > 0 so radius is > 0")
    }
}
