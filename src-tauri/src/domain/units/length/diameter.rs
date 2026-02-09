// domain/units/length/diameter.rs

use crate::domain::units::errors::UnitError;
use crate::domain::units::length::{Length, Radius};

/// Diameter stored internally as millimeters (mm).
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Diameter(f64);

impl Diameter {
    pub fn mm(value: f64) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NotFinite("Diameter"));
        }
        if value <= 0.0 {
            return Err(UnitError::NonPositiveValue("Diameter"));
        }
        Ok(Self(value))
    }

    pub fn mm_value(self) -> f64 {
        self.0
    }

    pub fn as_length(self) -> Length {
        // safe because diameter is always finite and > 0
        Length::mm(self.0).expect("Diameter is always valid length")
    }

    pub fn radius(self) -> Radius {
        Radius::mm(self.0 / 2.0).expect("Diameter is > 0 so radius is > 0")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::test_utils::approx::{approx_eq, DEFAULT_EPS};

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
    use crate::domain::test_utils::approx::{approx_eq, DEFAULT_EPS};
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
