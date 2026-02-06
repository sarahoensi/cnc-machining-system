// domain/units/machining/cutting_speed.rs

use std::f64::consts::PI;

use crate::domain::units::errors::UnitError;
use crate::domain::units::length::Diameter;
use crate::domain::units::motion::Rpm;

/// Surface cutting speed stored as meters per minute (m/min).
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct CuttingSpeed(f64);

impl CuttingSpeed {
    pub fn meters_per_min(value: f64) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NotFinite("CuttingSpeed"));
        }
        if value <= 0.0 {
            return Err(UnitError::NonPositiveValue("CuttingSpeed"));
        }
        Ok(Self(value))
    }

    pub fn meters_per_min_value(self) -> f64 {
        self.0
    }

    /// Vc = π * D * n / 1000
    /// D in mm, n in RPM => Vc in m/min
    pub fn from_rpm(diameter: Diameter, rpm: Rpm) -> Result<Self, UnitError> {
        let speed = PI * diameter.mm_value() * rpm.value() / 1000.0;
        Self::meters_per_min(speed)
    }

    /// n = (Vc * 1000) / (π * D)
    pub fn to_rpm(self, diameter: Diameter) -> Result<Rpm, UnitError> {
        let rpm = (self.0 * 1000.0) / (PI * diameter.mm_value());
        Rpm::new(rpm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::units::length::Diameter;
    use crate::domain::units::motion::Rpm;

    #[test]
    fn rejects_non_positive() {
        assert!(CuttingSpeed::meters_per_min(0.0).is_err());
        assert!(CuttingSpeed::meters_per_min(-1.0).is_err());
    }

    #[test]
    fn computes_from_rpm_roundtrip() {
        let d = Diameter::mm(10.0).unwrap();
        let rpm = Rpm::new(1000.0).unwrap();
        let speed = CuttingSpeed::from_rpm(d, rpm).unwrap();

        let rpm2 = speed.to_rpm(d).unwrap();
        assert!((rpm2.value() - 1000.0).abs() < 1e-6);
    }
}
