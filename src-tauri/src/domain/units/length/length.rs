// domain/units/length/length.rs

use crate::domain::units::errors::UnitError;

/// Linear length stored internally as millimeters (mm).
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Length(f64);

impl Length {
    pub fn mm(value: f64) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NotFinite("Length"));
        }
        Ok(Self(value))
    }

    pub fn mm_non_negative(value: f64) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NotFinite("Length"));
        }
        if value < 0.0 {
            return Err(UnitError::NegativeValue("Length"));
        }
        Ok(Self(value))
    }

    pub fn mm_positive(value: f64) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NotFinite("Length"));
        }
        if value <= 0.0 {
            return Err(UnitError::NonPositiveValue("Length"));
        }
        Ok(Self(value))
    }

    /// Raw value in millimeters.
    pub fn mm_value(self) -> f64 {
        self.0
    }

    /// Convenience conversion: inches -> mm.
    pub fn inches(value: f64) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NotFinite("Length"));
        }
        Ok(Self(value * 25.4))
    }

    pub fn inches_value(self) -> f64 {
        self.0 / 25.4
    }
}

// Small, safe arithmetic helpers (intentionally minimal)
impl std::ops::Add for Length {
    type Output = Length;
    fn add(self, rhs: Length) -> Length {
        Length(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Length {
    type Output = Length;
    fn sub(self, rhs: Length) -> Length {
        Length(self.0 - rhs.0)
    }
}

impl std::ops::Mul<f64> for Length {
    type Output = Length;
    fn mul(self, rhs: f64) -> Length {
        Length(self.0 * rhs)
    }
}

impl std::ops::Div<f64> for Length {
    type Output = Length;
    fn div(self, rhs: f64) -> Length {
        Length(self.0 / rhs)
    }
}
