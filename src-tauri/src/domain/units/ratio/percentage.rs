// domain/units/ratio/percenage.rs

use crate::domain::units::errors::UnitError;

/// Percentage stored internally as 0.0..=1.0 fraction.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Percentage(f64);

impl Percentage {
    /// 0..=100
    pub fn from_percent(value: f64) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NotFinite("Percentage"));
        }
        if value < 0.0 || value > 100.0 {
            return Err(UnitError::OutOfRange {
                ty: "Percentage",
                min: 0.0,
                max: 100.0,
                actual: value,
            });
        }
        Ok(Self(value / 100.0))
    }

    /// 0..=1
    pub fn from_fraction(value: f64) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NotFinite("Percentage"));
        }
        if value < 0.0 || value > 1.0 {
            return Err(UnitError::OutOfRange {
                ty: "Percentage",
                min: 0.0,
                max: 1.0,
                actual: value,
            });
        }
        Ok(Self(value))
    }

    pub fn fraction_value(self) -> f64 {
        self.0
    }

    pub fn percent_value(self) -> f64 {
        self.0 * 100.0
    }
}

// ------------- TESTS ---------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};

    // --- Validation percent ---

    #[test]
    fn percent_rejects_nan_and_infinity() {
        assert!(Percentage::from_percent(f64::NAN).is_err());
        assert!(Percentage::from_percent(f64::INFINITY).is_err());
    }

    #[test]
    fn percent_rejects_out_of_range() {
        assert!(Percentage::from_percent(-1.0).is_err());
        assert!(Percentage::from_percent(101.0).is_err());
    }

    #[test]
    fn percent_accepts_boundaries() {
        assert!(Percentage::from_percent(0.0).is_ok());
        assert!(Percentage::from_percent(100.0).is_ok());
    }

    // --- Validation fraction ---

    #[test]
    fn fraction_rejects_out_of_range() {
        assert!(Percentage::from_fraction(-0.1).is_err());
        assert!(Percentage::from_fraction(1.1).is_err());
    }

    #[test]
    fn fraction_accepts_boundaries() {
        assert!(Percentage::from_fraction(0.0).is_ok());
        assert!(Percentage::from_fraction(1.0).is_ok());
    }

    // --- Conversion correctness ---

    #[test]
    fn percent_to_fraction_conversion() {
        let p = Percentage::from_percent(25.0).unwrap();

        assert!(approx_eq(p.fraction_value(), 0.25, DEFAULT_EPS));
    }

    #[test]
    fn fraction_to_percent_conversion() {
        let p = Percentage::from_fraction(0.75).unwrap();

        assert!(approx_eq(p.percent_value(), 75.0, DEFAULT_EPS));
    }

    // --- Round trip invariants ---

    #[test]
    fn percent_round_trip() {
        let original = 42.5;

        let p = Percentage::from_percent(original).unwrap();
        let result = p.percent_value();

        assert!(approx_eq(original, result, DEFAULT_EPS));
    }

    #[test]
    fn fraction_round_trip() {
        let original = 0.42;

        let p = Percentage::from_fraction(original).unwrap();
        let result = p.fraction_value();

        assert!(approx_eq(original, result, DEFAULT_EPS));
    }

    // --- Ordering ---

    #[test]
    fn ordering_works() {
        let a = Percentage::from_percent(20.0).unwrap();
        let b = Percentage::from_percent(80.0).unwrap();

        assert!(a < b);
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};
    use proptest::prelude::*;

    // --- percent round trip ---
    proptest! {
        #[test]
        fn percent_round_trip_property(value in 0.0f64..100.0f64) {
            let p = Percentage::from_percent(value).unwrap();

            prop_assert!(approx_eq(
                p.percent_value(),
                value,
                DEFAULT_EPS
            ));
        }
    }

    // --- fraction round trip ---
    proptest! {
        #[test]
        fn fraction_round_trip_property(value in 0.0f64..1.0f64) {
            let p = Percentage::from_fraction(value).unwrap();

            prop_assert!(approx_eq(
                p.fraction_value(),
                value,
                DEFAULT_EPS
            ));
        }
    }

    // --- conversion symmetry ---
    proptest! {
        #[test]
        fn percent_fraction_consistency(value in 0.0f64..100.0f64) {
            let p = Percentage::from_percent(value).unwrap();

            let p2 = Percentage::from_fraction(p.fraction_value()).unwrap();

            prop_assert!(approx_eq(
                p.percent_value(),
                p2.percent_value(),
                DEFAULT_EPS
            ));
        }
    }

    // --- finite invariant ---
    proptest! {
        #[test]
        fn value_stays_finite(value in 0.0f64..100.0f64) {
            let p = Percentage::from_percent(value).unwrap();

            prop_assert!(p.fraction_value().is_finite());
        }
    }
}
