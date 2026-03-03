// domain/units/length/pitch.rs

use crate::domain::units::{UnitsError, length::{
    Length, PositiveLength
}};

/// Represents a strictly positive linear pitch (mm per revolution).
///
/// Semantically distinct from generic length,
/// but physically represented as a positive scalar value.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Pitch(PositiveLength);

impl Pitch {
    /// Creates a Pitch from millimeters per revolution.
    ///
    /// # Errors
    /// Returns error if value is not finite or ≤ 0.
    pub fn mm_per_rev(value: f64) -> Result<Self, UnitsError> {
        Ok(Self(PositiveLength::mm(value)?))
    }

    /// Returns pitch value in mm per revolution.
    pub fn mm_per_rev_value(self) -> f64 {
        self.0.mm_value()
    }

    /// Returns underlying PositiveLength.
    pub fn as_positive_length(self) -> PositiveLength {
        self.0
    }

    /// Converts pitch to a signed Length representing
    /// linear travel per single revolution.
    pub fn as_length_per_rev(self) -> Length {
        self.0.as_length()
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
