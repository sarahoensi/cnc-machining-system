// domain/units/machining/chip_load.rs

use crate::domain::units::{machining::MachiningUnitError};

/// Represents chip load per cutting tooth.
///
/// Stored internally as millimeters per tooth (mm/tooth).
/// Values must be finite and strictly positive.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct ChipLoad(f64);

impl ChipLoad {
    /// Creates a [`ChipLoad`] from millimeters per tooth.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not finite or is less than or equal to zero.
    pub fn mm_per_tooth(value: f64) -> Result<Self, MachiningUnitError> {
        if !value.is_finite() {
            return Err(MachiningUnitError::NotFinite { value });
        }
        if value <= 0.0 {
            return Err(MachiningUnitError::NonPositive { value });
        }
        Ok(Self(value))
    }

    /// Returns the chip load value in millimeters per tooth.
    pub  fn mm_per_tooth_value(self) -> f64 {
        self.0
    }
}

// -------------- TESTS ---------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};

    // --- Validation ---

    #[test]
    fn rejects_nan() {
        assert!(ChipLoad::mm_per_tooth(f64::NAN).is_err());
    }

    #[test]
    fn rejects_infinity() {
        assert!(ChipLoad::mm_per_tooth(f64::INFINITY).is_err());
        assert!(ChipLoad::mm_per_tooth(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn rejects_zero() {
        assert!(ChipLoad::mm_per_tooth(0.0).is_err());
    }

    #[test]
    fn rejects_negative() {
        assert!(ChipLoad::mm_per_tooth(-1.0).is_err());
    }

    // --- Value round trip ---

    #[test]
    fn value_round_trip() {
        let c = ChipLoad::mm_per_tooth(0.05).unwrap();

        assert!(approx_eq(
            c.mm_per_tooth_value(),
            0.05,
            DEFAULT_EPS
        ));
    }

    // --- Ordering ---

    #[test]
    fn ordering_works() {
        let a = ChipLoad::mm_per_tooth(0.02).unwrap();
        let b = ChipLoad::mm_per_tooth(0.05).unwrap();

        assert!(a < b);
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};
    use proptest::prelude::*;

    // --- round trip ---
    proptest! {
        #[test]
        fn round_trip_property(value in 1e-9f64..1e3f64) {
            let c = ChipLoad::mm_per_tooth(value).unwrap();

            prop_assert!(approx_eq(
                c.mm_per_tooth_value(),
                value,
                DEFAULT_EPS
            ));
        }
    }

    // --- ordering preserved ---
    proptest! {
        #[test]
        fn ordering_preserved(a in 1e-9f64..1e3f64,
                             b in 1e-9f64..1e3f64) {

            prop_assume!(!approx_eq(a, b, DEFAULT_EPS));

            let (a, b) = if a < b { (a, b) } else { (b, a) };

            let a = ChipLoad::mm_per_tooth(a).unwrap();
            let b = ChipLoad::mm_per_tooth(b).unwrap();

            prop_assert!(a < b);
        }
    }

    // --- finite invariant ---
    proptest! {
        #[test]
        fn value_stays_finite(value in 1e-9f64..1e3f64) {
            let c = ChipLoad::mm_per_tooth(value).unwrap();

            prop_assert!(c.mm_per_tooth_value().is_finite());
        }
    }
}
