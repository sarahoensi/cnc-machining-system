// domain/units/motion/rpm.rs

use crate::domain::units::errors::UnitError;

/// Represents rotational speed in revolutions per minute (RPM).
///
/// Values must be finite and strictly positive.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Rpm(f64);

impl Rpm {
    /// Creates a new [`Rpm`] value.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not finite or is less than or equal to zero.
    pub fn new(value: f64) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NotFinite("Rpm"));
        }
        if value <= 0.0 {
            return Err(UnitError::NonPositiveValue("Rpm"));
        }
        Ok(Self(value))
    }

    /// Returns the RPM value.
    pub fn value(self) -> f64 {
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
        assert!(Rpm::new(f64::NAN).is_err());
    }

    #[test]
    fn rejects_infinity() {
        assert!(Rpm::new(f64::INFINITY).is_err());
        assert!(Rpm::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn rejects_zero() {
        assert!(Rpm::new(0.0).is_err());
    }

    #[test]
    fn rejects_negative() {
        assert!(Rpm::new(-100.0).is_err());
    }

    // --- Value round trip ---

    #[test]
    fn value_round_trip() {
        let r = Rpm::new(1200.0).unwrap();

        assert!(approx_eq(
            r.value(),
            1200.0,
            DEFAULT_EPS
        ));
    }

    // --- Ordering ---

    #[test]
    fn ordering_works() {
        let a = Rpm::new(500.0).unwrap();
        let b = Rpm::new(1500.0).unwrap();

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
        fn round_trip_property(value in 1e-9f64..1e7f64) {
            let r = Rpm::new(value).unwrap();

            prop_assert!(approx_eq(
                r.value(),
                value,
                DEFAULT_EPS
            ));
        }
    }

    // --- ordering preserved ---
    proptest! {
        #[test]
        fn ordering_preserved(a in 1e-9f64..1e7f64,
                             b in 1e-9f64..1e7f64) {

            prop_assume!(!approx_eq(a, b, DEFAULT_EPS));

            let (a, b) = if a < b { (a, b) } else { (b, a) };

            let a = Rpm::new(a).unwrap();
            let b = Rpm::new(b).unwrap();

            prop_assert!(a < b);
        }
    }

    // --- finite invariant ---
    proptest! {
        #[test]
        fn value_stays_finite(value in 1e-9f64..1e7f64) {
            let r = Rpm::new(value).unwrap();

            prop_assert!(r.value().is_finite());
        }
    }
}
