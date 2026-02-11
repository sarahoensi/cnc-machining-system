// domain/units/machining/cutting_speed.rs

use std::f64::consts::PI;

use crate::domain::units::errors::UnitError;
use crate::domain::units::{Diameter, Rpm};

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

// -------------- TESTS ---------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};

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

    #[test]
    fn rpm_roundtrip() {
        let d = Diameter::mm(10.0).unwrap();
        let rpm = Rpm::new(1000.0).unwrap();

        let speed = CuttingSpeed::from_rpm(d, rpm).unwrap();
        let rpm2 = speed.to_rpm(d).unwrap();

        assert!(approx_eq(
            rpm2.value(),
            rpm.value(),
            DEFAULT_EPS
        ));
    }

    #[test]
    fn increases_with_rpm() {
        let d = Diameter::mm(10.0).unwrap();

        let low = CuttingSpeed::from_rpm(d, Rpm::new(500.0).unwrap()).unwrap();
        let high = CuttingSpeed::from_rpm(d, Rpm::new(1000.0).unwrap()).unwrap();

        assert!(low < high);
    }

    #[test]
    fn increases_with_diameter() {
        let rpm = Rpm::new(1000.0).unwrap();

        let small = CuttingSpeed::from_rpm(Diameter::mm(5.0).unwrap(), rpm).unwrap();
        let large = CuttingSpeed::from_rpm(Diameter::mm(10.0).unwrap(), rpm).unwrap();

        assert!(small < large);
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn rpm_roundtrip_property(
            diameter in 1e-6f64..1e4f64,
            rpm in 1e-6f64..1e6f64
        ) {

            let d = Diameter::mm(diameter).unwrap();
            let rpm = Rpm::new(rpm).unwrap();

            let speed = CuttingSpeed::from_rpm(d, rpm).unwrap();
            let rpm2 = speed.to_rpm(d).unwrap();

            prop_assert!(approx_eq(
                rpm.value(),
                rpm2.value(),
                DEFAULT_EPS
            ));
        }
    }

    proptest! {
        #[test]
        fn speed_is_finite(
            diameter in 1e-6f64..1e4f64,
            rpm in 1e-6f64..1e6f64
        ) {
            let d = Diameter::mm(diameter).unwrap();
            let rpm = Rpm::new(rpm).unwrap();

            let speed = CuttingSpeed::from_rpm(d, rpm).unwrap();

            prop_assert!(speed.meters_per_min_value().is_finite());
        }
    }
}

