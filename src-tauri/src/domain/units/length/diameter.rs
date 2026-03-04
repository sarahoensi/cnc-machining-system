// domain/units/length/diameter.rs

use crate::domain::units::length::{Length, PositiveLength, };
use crate::domain::units::{Radius, UnitsError};

/// Represents a strictly positive diameter measurement.
///
/// Semantically distinct from generic length, but
/// physically represented as a positive length.
#[must_use]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Diameter(PositiveLength);

impl Diameter {
    /// Internal constructor used by domain math.
    pub(crate) fn mm_unchecked(value: f64) -> Self {
        Self(PositiveLength::mm_unchecked(value))
    }

    /// Creates a Diameter from millimeters.
    ///
    /// # Errors
    /// Returns error if value is not finite or ≤ 0.
    pub fn mm(value: f64) -> Result<Self, UnitsError> {
        Ok(Self(PositiveLength::mm(value)?))
    }

    /// Returns diameter value in millimeters.
    pub fn mm_value(self) -> f64 {
        self.0.mm_value()
    }

    /// Returns underlying PositiveLength.
    pub fn as_positive_length(self) -> PositiveLength {
        self.0
    }

    /// Converts to signed Length.
    pub fn as_length(self) -> Length {
        self.0.as_length()
    }

    /// Computes corresponding Radius.
    pub fn radius(self) -> Radius {
        // safe: half of positive length is still positive
        Radius::mm(self.mm_value() / 2.0)
            .expect("Invariant violation: radius must be positive")
    }
}

// ------------------------ TESTS ------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};

    // --- Validation ---

    #[test]
    fn rejects_nan() {
        assert!(Diameter::mm(f64::NAN).is_err());
    }

    #[test]
    fn rejects_infinity() {
        assert!(Diameter::mm(f64::INFINITY).is_err());
        assert!(Diameter::mm(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn rejects_zero() {
        assert!(Diameter::mm(0.0).is_err());
    }

    #[test]
    fn rejects_negative() {
        assert!(Diameter::mm(-5.0).is_err());
    }

    // --- Basic correctness ---

    #[test]
    fn mm_value_round_trip() {
        let d = Diameter::mm(10.0).unwrap();
        assert!(approx_eq(d.mm_value(), 10.0, DEFAULT_EPS));
    }

    // --- Conversion to Length ---

    #[test]
    fn converts_to_length() {
        let d = Diameter::mm(12.0).unwrap();
        let l = d.as_length();

        assert!(approx_eq(l.mm_value(), 12.0, DEFAULT_EPS));
    }

    // --- Conversion to Radius ---

    #[test]
    fn radius_relation() {
        let d = Diameter::mm(20.0).unwrap();
        let r = d.radius();

        assert!(approx_eq(r.mm_value(), 10.0, DEFAULT_EPS));
    }

    // --- Ordering ---

    #[test]
    fn ordering_works() {
        let a = Diameter::mm(5.0).unwrap();
        let b = Diameter::mm(10.0).unwrap();

        assert!(a < b);
    }

    // -- range --
    #[test]
fn accepts_min_positive_value() {
    assert!(Diameter::mm(f64::MIN_POSITIVE).is_ok());
}

}

#[cfg(test)]
mod property_tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};
    use proptest::prelude::*;

    // --- radius invariant ---
    proptest! {
        #[test]
        fn radius_is_half_diameter(value in 1e-9f64..1e6f64) {
            let d = Diameter::mm(value).unwrap();
            let r = d.radius();

            prop_assert!(approx_eq(
                r.mm_value() * 2.0,
                d.mm_value(),
                DEFAULT_EPS
            ));
        }
    }

    // --- as_length invariant ---
    proptest! {
        #[test]
        fn as_length_preserves_value(value in 1e-9f64..1e6f64) {
            let d = Diameter::mm(value).unwrap();
            let l = d.as_length();

            prop_assert!(approx_eq(
                l.mm_value(),
                d.mm_value(),
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

            let a = Diameter::mm(a).unwrap();
            let b = Diameter::mm(b).unwrap();

            prop_assert!(a < b);
        }
    }

}
