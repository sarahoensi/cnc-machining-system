// domain/units/length/pitch.rs

use crate::domain::units::errors::UnitError;
use crate::domain::units::length::Length;

/// Pitch stored internally as millimeters per revolution (mm/rev).
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Pitch(f64);

impl Pitch {
    pub fn mm_per_rev(value: f64) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NotFinite("Pitch"));
        }
        if value <= 0.0 {
            return Err(UnitError::NonPositiveValue("Pitch"));
        }
        Ok(Self(value))
    }

    pub fn mm_per_rev_value(self) -> f64 {
        self.0
    }

    pub fn as_length_per_rev(self) -> Length {
        Length::mm(self.0).expect("Pitch is always valid length")
    }
}
