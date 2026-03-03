// domain/units/length/radius.rs

use crate::domain::units::length::{
    Length,
    PositiveLength,
    LengthUnitError,
};

use crate::domain::units::Diameter;

/// Represents a strictly positive radius measurement.
///
/// Semantically distinct from generic length,
/// but physically represented as a positive length.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Radius(PositiveLength);

impl Radius {
    /// Creates a Radius from millimeters.
    ///
    /// # Errors
    /// Returns error if value is not finite or ≤ 0.
    pub fn mm(value: f64) -> Result<Self, LengthUnitError> {
        Ok(Self(PositiveLength::mm(value)?))
    }

    /// Returns radius value in millimeters.
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

    /// Computes corresponding Diameter.
    pub fn diameter(self) -> Diameter {
        // 2 * positive > 0
        Diameter::mm(self.mm_value() * 2.0)
            .expect("Invariant violation: diameter must be positive")
    }
}

// ----------------- TESTS -----------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};

    // --- Validation ---

    #[test]
    fn rejects_nan() {
        assert!(Radius::mm(f64::NAN).is_err());
    }

    #[test]
    fn rejects_infinity() {
        assert!(Radius::mm(f64::INFINITY).is_err());
        assert!(Radius::mm(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn rejects_zero() {
        assert!(Radius::mm(0.0).is_err());
    }

    #[test]
    fn rejects_negative() {
        assert!(Radius::mm(-1.0).is_err());
    }

    // --- Basic round trip ---

    #[test]
    fn mm_value_round_trip() {
        let r = Radius::mm(5.0).unwrap();

        assert!(approx_eq(
            r.mm_value(),
            5.0,
            DEFAULT_EPS
        ));
    }

    // --- Conversion to Length ---

    #[test]
    fn converts_to_length() {
        let r = Radius::mm(3.0).unwrap();
        let l = r.as_length();

        assert!(approx_eq(
            l.mm_value(),
            3.0,
            DEFAULT_EPS
        ));
    }

    // --- Conversion to Diameter ---

    #[test]
    fn diameter_relation() {
        let r = Radius::mm(4.0).unwrap();
        let d = r.diameter();

        assert!(approx_eq(
            d.mm_value(),
            8.0,
            DEFAULT_EPS
        ));
    }

    // --- Ordering ---

    #[test]
    fn ordering_works() {
        let a = Radius::mm(2.0).unwrap();
        let b = Radius::mm(4.0).unwrap();

        assert!(a < b);
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};
    use proptest::prelude::*;

    // --- diameter invariant ---
    proptest! {
        #[test]
        fn diameter_is_double_radius(value in 1e-9f64..1e6f64) {
            let r = Radius::mm(value).unwrap();
            let d = r.diameter();

            prop_assert!(approx_eq(
                d.mm_value(),
                r.mm_value() * 2.0,
                DEFAULT_EPS
            ));
        }
    }

    // --- round trip radius -> diameter -> radius ---
    proptest! {
        #[test]
        fn round_trip_via_diameter(value in 1e-9f64..1e6f64) {
            let r = Radius::mm(value).unwrap();
            let d = r.diameter();
            let r2 = d.radius();

            prop_assert!(approx_eq(
                r.mm_value(),
                r2.mm_value(),
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

            let a = Radius::mm(a).unwrap();
            let b = Radius::mm(b).unwrap();

            prop_assert!(a < b);
        }
    }

    // --- finite propagation ---
    proptest! {
        #[test]
        fn conversion_stays_finite(value in 1e-9f64..1e6f64) {
            let r = Radius::mm(value).unwrap();

            prop_assert!(r.diameter().mm_value().is_finite());
        }
    }
}

