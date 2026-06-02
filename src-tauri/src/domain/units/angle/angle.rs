// domain/units/angle/angle.rs

use super::error::AngleError;
use crate::domain::units::{core::NumericError, UnitsError};

#[must_use]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Angle(f64);

#[must_use]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct AcuteAngle(f64);

// ============================================================
// Angle (signed, finite)
// ============================================================

impl Angle {
    /// Internal constructor used by domain math.
    #[allow(dead_code)]
    pub(crate) fn radians_unchecked(value: f64) -> Self {
        debug_assert!(value.is_finite());
        Self(value)
    }

    fn validate_finite(value: f64) -> Result<f64, NumericError> {
        if value.is_finite() {
            Ok(value)
        } else {
            Err(NumericError::NotFinite(value))
        }
    }

    pub fn radians(value: f64) -> Result<Self, UnitsError> {
        Ok(Self(Self::validate_finite(value)?))
    }

    pub fn degrees(value: f64) -> Result<Self, UnitsError> {
        Ok(Self(Self::validate_finite(value)?.to_radians()))
    }

    pub fn radians_value(self) -> f64 {
        self.0
    }

    pub fn degrees_value(self) -> f64 {
        self.0.to_degrees()
    }

    pub fn try_into_acute(self) -> Result<AcuteAngle, UnitsError> {
        AcuteAngle::radians(self.0)
    }
}

// ============================================================
// AcuteAngle (0 < θ < 90°)
// ============================================================

impl AcuteAngle {
    const HALF_PI: f64 = std::f64::consts::FRAC_PI_2;

    /// Internal constructor used by domain math.
    pub(crate) fn radians_unchecked(value: f64) -> Self {
        debug_assert!(value.is_finite());
        debug_assert!(value > 0.0);
        debug_assert!(value < Self::HALF_PI);
        Self(value)
    }

    fn validate_finite(value: f64) -> Result<f64, NumericError> {
        if value.is_finite() {
            Ok(value)
        } else {
            Err(NumericError::NotFinite(value))
        }
    }

    pub fn radians(value: f64) -> Result<Self, UnitsError> {
        let value = Self::validate_finite(value)?;

        if value <= 0.0 || value >= Self::HALF_PI {
            return Err(AngleError::NotAcute(value.to_degrees()).into());
        }

        Ok(Self(value))
    }

    pub fn degrees(value: f64) -> Result<Self, UnitsError> {
        let value = Self::validate_finite(value)?;

        if value <= 0.0 || value >= 90.0 {
            return Err(AngleError::NotAcute(value).into());
        }

        Ok(Self(value.to_radians()))
    }

    pub fn radians_value(self) -> f64 {
        self.0
    }

    pub fn degrees_value(self) -> f64 {
        self.0.to_degrees()
    }

    pub fn as_angle(self) -> Angle {
        Angle(self.0)
    }
}
