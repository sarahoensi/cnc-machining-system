// domain/units/machining/cutting_speed.rs



use crate::domain::units::machining::MachiningUnitError;
use crate::domain::units::{PositiveScalar};

/// Represents surface cutting speed.
///
/// Stored internally as meters per minute (m/min).
/// Values must be finite and strictly positive.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct CuttingSpeed(PositiveScalar);

impl CuttingSpeed {
    pub fn meters_per_min(value: f64) -> Result<Self, MachiningUnitError> {
        Ok(Self(PositiveScalar::new(
            value,
            MachiningUnitError::NotFinite,
            MachiningUnitError::NonPositive,
        )?))
    }

    pub fn meters_per_min_value(self) -> f64 {
        self.0.value()
    }
}

// -------------- TESTS ---------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_positive() {
        assert!(CuttingSpeed::meters_per_min(0.0).is_err());
        assert!(CuttingSpeed::meters_per_min(-1.0).is_err());
    }

    #[test]
    fn rejects_nan_and_infinity() {
        assert!(CuttingSpeed::meters_per_min(f64::NAN).is_err());
        assert!(CuttingSpeed::meters_per_min(f64::INFINITY).is_err());
    }

}