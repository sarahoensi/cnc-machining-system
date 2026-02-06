// domain/units/ratio/percenage.rs

use crate::domain::units::errors::UnitError;

/// Percentage stored internally as 0.0..=1.0 fraction.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Percentage(f64);

impl Percentage {
    /// 0..=100
    pub fn from_percent(value: f64) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NotFinite("Percentage"));
        }
        if value < 0.0 || value > 100.0 {
            return Err(UnitError::OutOfRange {
                ty: "Percentage",
                min: 0.0,
                max: 100.0,
                actual: value,
            });
        }
        Ok(Self(value / 100.0))
    }

    /// 0..=1
    pub fn from_fraction(value: f64) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NotFinite("Percentage"));
        }
        if value < 0.0 || value > 1.0 {
            return Err(UnitError::OutOfRange {
                ty: "Percentage",
                min: 0.0,
                max: 1.0,
                actual: value,
            });
        }
        Ok(Self(value))
    }

    pub fn fraction_value(self) -> f64 {
        self.0
    }

    pub fn percent_value(self) -> f64 {
        self.0 * 100.0
    }
}
