// units/angle/angle.rs

use crate::domain::units::errors::UnitError;

/// Angle stored internally as radians.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Angle(f64);

impl Angle {
    fn validate_finite(value: f64) -> Result<f64, UnitError> {
        if value.is_finite() {
            Ok(value)
        } else {
            Err(UnitError::NotFinite("Angle"))
        }
    }

    pub fn radians(value: f64) -> Result<Self, UnitError> {
        Ok(Self(Self::validate_finite(value)?))
    }

    pub fn degrees(value: f64) -> Result<Self, UnitError> {
        Ok(Self(Self::validate_finite(value)?.to_radians()))
    }

    pub fn radians_value(self) -> f64 {
        self.0
    }

    pub fn degrees_value(self) -> f64 {
        self.0.to_degrees()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::test_utils::approx::{approx_eq, DEFAULT_EPS};

    // --- Validation tests ---

    #[test]
    fn rejects_nan() {
        assert!(Angle::radians(f64::NAN).is_err());
        assert!(Angle::degrees(f64::NAN).is_err());
    }

    #[test]
    fn rejects_infinity() {
        assert!(Angle::radians(f64::INFINITY).is_err());
        assert!(Angle::radians(f64::NEG_INFINITY).is_err());

        assert!(Angle::degrees(f64::INFINITY).is_err());
        assert!(Angle::degrees(f64::NEG_INFINITY).is_err());
    }

    // --- Conversion correctness ---

    #[test]
    fn degrees_to_radians_basic_values() {
        let a = Angle::degrees(180.0).unwrap();
        assert!(approx_eq(
            a.radians_value(),
            std::f64::consts::PI,
            DEFAULT_EPS
        ));

        let a = Angle::degrees(90.0).unwrap();
        assert!(approx_eq(
            a.radians_value(),
            std::f64::consts::FRAC_PI_2,
            DEFAULT_EPS
        ));
    }

    #[test]
    fn radians_to_degrees_basic_values() {
        let a = Angle::radians(std::f64::consts::PI).unwrap();
        assert!(approx_eq(a.degrees_value(), 180.0, DEFAULT_EPS));

        let a = Angle::radians(std::f64::consts::FRAC_PI_2).unwrap();
        assert!(approx_eq(a.degrees_value(), 90.0, DEFAULT_EPS));
    }

    // --- Round trip invariants ---

    #[test]
    fn degrees_round_trip() {
        let original = 123.456;

        let angle = Angle::degrees(original).unwrap();
        let result = angle.degrees_value();

        assert!(approx_eq(original, result, DEFAULT_EPS));
    }

    #[test]
    fn radians_round_trip() {
        let original = 1.2345;

        let angle = Angle::radians(original).unwrap();
        let result = angle.radians_value();

        assert!(approx_eq(original, result, DEFAULT_EPS));
    }

    // --- Negative values ---

    #[test]
    fn supports_negative_angles() {
        let a = Angle::degrees(-90.0).unwrap();
        assert!(approx_eq(
            a.radians_value(),
            -std::f64::consts::FRAC_PI_2,
            DEFAULT_EPS
        ));
    }

    // --- Ordering / comparisons ---

    #[test]
    fn comparison_works() {
        let a = Angle::degrees(30.0).unwrap();
        let b = Angle::degrees(60.0).unwrap();

        assert!(a < b);

        let a = Angle::degrees(45.0).unwrap();
        let b = Angle::degrees(45.0).unwrap();
        assert_eq!(a, b);
    }

    // -- zero identity ---
    #[test]
fn zero_is_identity() {
    let a = Angle::degrees(0.0).unwrap();
    assert_eq!(a.radians_value(), 0.0);

    let a = Angle::radians(0.0).unwrap();
    assert_eq!(a.degrees_value(), 0.0);
}

}

#[cfg(test)]
mod property_tests {
    use super::*;
    use crate::domain::test_utils::approx::{approx_eq, DEFAULT_EPS};
    use proptest::prelude::*;

    // --- Round trip: degrees -> radians -> degrees ---
    proptest! {
        #[test]
        fn degrees_round_trip_property(value in -1.0e6f64..1.0e6f64) {
            let angle = Angle::degrees(value).unwrap();
            let result = angle.degrees_value();

            prop_assert!(approx_eq(value, result, DEFAULT_EPS));
        }
    }

    // --- Round trip: radians -> degrees -> radians ---
    proptest! {
        #[test]
        fn radians_round_trip_property(value in -1.0e6f64..1.0e6f64) {
            let angle = Angle::radians(value).unwrap();
            let result = angle.radians_value();

            prop_assert!(approx_eq(value, result, DEFAULT_EPS));
        }
    }

    // --- Ordering invariant ---
    proptest! {
    #[test]
    fn ordering_preserved(a in -1.0e6f64..1.0e6f64,
                         b in -1.0e6f64..1.0e6f64) {

        prop_assume!(a != b);

        let (a, b) = if a < b { (a, b) } else { (b, a) };

        let a = Angle::degrees(a).unwrap();
        let b = Angle::degrees(b).unwrap();

        prop_assert!(a < b);
    }
}


    // --- Finite invariant ---
    proptest! {
        #[test]
        fn always_produces_finite(value in any::<f64>().prop_filter(
            "finite only",
            |v| v.is_finite()
        )) {

            let a = Angle::degrees(value).unwrap();
            prop_assert!(a.radians_value().is_finite());
        }
    }
}
