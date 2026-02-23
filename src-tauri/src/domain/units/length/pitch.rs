// domain/units/length/pitch.rs

use super::error::LengthUnitError;
use crate::domain::units::Length;

/// Represents a linear pitch measurement.
///
/// Stored internally as millimeters per revolution (mm/rev).
/// Values must be finite and strictly positive.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Pitch(f64);

impl Pitch {
    /// Creates a [`Pitch`] from millimeters per revolution.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not finite or is less than or equal to zero.
    pub fn mm_per_rev(value: f64) -> Result<Self, LengthUnitError> {
        if !value.is_finite() {
            return Err(LengthUnitError::NotFinite { value });
        }
        if value <= 0.0 {
            return Err(LengthUnitError::NonPositive { value });
        }
        Ok(Self(value))
    }

    /// Returns the pitch value in millimeters per revolution.
    pub fn mm_per_rev_value(self) -> f64 {
        self.0
    }

    /// Converts the pitch to a [`Length`] representing linear travel per revolution.
    pub fn as_length_per_rev(self) -> Length {
        Length::mm(self.0).expect("Pitch is always valid length")
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
        assert!(Pitch::mm_per_rev(f64::NAN).is_err());
    }

    #[test]
    fn rejects_infinity() {
        assert!(Pitch::mm_per_rev(f64::INFINITY).is_err());
        assert!(Pitch::mm_per_rev(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn rejects_zero() {
        assert!(Pitch::mm_per_rev(0.0).is_err());
    }

    #[test]
    fn rejects_negative() {
        assert!(Pitch::mm_per_rev(-1.0).is_err());
    }

    // --- Basic round trip ---

    #[test]
    fn mm_per_rev_round_trip() {
        let p = Pitch::mm_per_rev(2.5).unwrap();

        assert!(approx_eq(p.mm_per_rev_value(), 2.5, DEFAULT_EPS));
    }

    // --- Conversion ---

    #[test]
    fn converts_to_length() {
        let p = Pitch::mm_per_rev(3.0).unwrap();
        let l = p.as_length_per_rev();

        assert!(approx_eq(l.mm_value(), 3.0, DEFAULT_EPS));
    }

    // --- Ordering ---

    #[test]
    fn ordering_works() {
        let a = Pitch::mm_per_rev(1.0).unwrap();
        let b = Pitch::mm_per_rev(2.0).unwrap();

        assert!(a < b);
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};
    use proptest::prelude::*;

    // --- conversion invariant ---
    proptest! {
        #[test]
        fn length_conversion_preserves_value(value in 1e-9f64..1e6f64) {
            let p = Pitch::mm_per_rev(value).unwrap();
            let l = p.as_length_per_rev();

            prop_assert!(approx_eq(
                l.mm_value(),
                p.mm_per_rev_value(),
                DEFAULT_EPS
            ));
        }
    }

    // --- ordering preserved ---
    proptest! {
        #[test]
        fn ordering_preserved(a in 1e-9f64..1e6f64,
                             b in 1e-9f64..1e6f64) {

            prop_assume!(!approx_eq(a, b, DEFAULT_EPS));

            let (a, b) = if a < b { (a, b) } else { (b, a) };

            let a = Pitch::mm_per_rev(a).unwrap();
            let b = Pitch::mm_per_rev(b).unwrap();

            prop_assert!(a < b);
        }
    }

    // --- always finite ---
    proptest! {
        #[test]
        fn conversion_stays_finite(value in 1e-9f64..1e6f64) {
            let p = Pitch::mm_per_rev(value).unwrap();

            prop_assert!(p.as_length_per_rev().mm_value().is_finite());
        }
    }
}
