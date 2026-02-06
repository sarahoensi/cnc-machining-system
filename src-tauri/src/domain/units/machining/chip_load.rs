// domain/units/machining/chip_load.rs

use crate::domain::units::errors::UnitError;

/// Chip load stored as millimeters per tooth (mm/tooth).
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct ChipLoad(f64);

impl ChipLoad {
    pub fn mm_per_tooth(value: f64) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NotFinite("ChipLoad"));
        }
        if value <= 0.0 {
            return Err(UnitError::NonPositiveValue("ChipLoad"));
        }
        Ok(Self(value))
    }

    pub fn mm_per_tooth_value(self) -> f64 {
        self.0
    }
}
