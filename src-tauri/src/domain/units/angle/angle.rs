// units/angle/angle.rs

use super::error::AngleError;

/// Represents a mathematical angle.
///
/// Stored internally in radians.
/// Values must be finite.
/// Negative angles are allowed.
/// The value is not normalized.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Angle(f64);

/// Represents a strictly acute angle (0 < θ < 90°).
///
/// Stored internally in radians.
/// Values must be finite and strictly between 0 and π/2.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct AcuteAngle(f64);



// ============================================================
// Angle (signed, finite)
// ============================================================

impl Angle {
    fn validate_finite(value: f64) -> Result<f64, AngleError> {
        if value.is_finite() {
            Ok(value)
        } else {
            Err(AngleError::NotFinite { value })
        }
    }

    /// Creates an angle from radians.
    pub fn radians(value: f64) -> Result<Self, AngleError> {
        Ok(Self(Self::validate_finite(value)?))
    }

    /// Creates an angle from degrees.
    pub fn degrees(value: f64) -> Result<Self, AngleError> {
        Ok(Self(Self::validate_finite(value)?.to_radians()))
    }

    /// Returns radians.
    pub fn radians_value(self) -> f64 {
        self.0
    }

    /// Returns degrees.
    pub fn degrees_value(self) -> f64 {
        self.0.to_degrees()
    }

    /// Attempts to convert into an AcuteAngle.
    pub fn try_into_acute(self) -> Result<AcuteAngle, AngleError> {
        AcuteAngle::radians(self.0)
    }
}



// ============================================================
// AcuteAngle (0 < θ < 90°)
// ============================================================

impl AcuteAngle {
    const HALF_PI: f64 = std::f64::consts::FRAC_PI_2;

    /// Creates an acute angle from radians.
    pub fn radians(value: f64) -> Result<Self, AngleError> {
        if !value.is_finite() {
            return Err(AngleError::NotFinite { value });
        }

        if value <= 0.0 || value >= Self::HALF_PI {
            return Err(AngleError::NotAcute {
                degrees: value.to_degrees(),
            });
        }

        Ok(Self(value))
    }

    /// Creates an acute angle from degrees.
    pub fn degrees(value: f64) -> Result<Self, AngleError> {
        if !value.is_finite() {
            return Err(AngleError::NotFinite { value });
        }

        if value <= 0.0 || value >= 90.0 {
            return Err(AngleError::NotAcute { degrees: value });
        }

        Ok(Self(value.to_radians()))
    }

    /// Returns radians.
    pub fn radians_value(self) -> f64 {
        self.0
    }

    /// Returns degrees.
    pub fn degrees_value(self) -> f64 {
        self.0.to_degrees()
    }

    /// Converts back to general Angle.
    pub fn as_angle(self) -> Angle {
        Angle(self.0)
    }
}

// -------------------- TESTS --------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};

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
        assert!(approx_eq(a.radians_value(), 0.0, DEFAULT_EPS));

        let a = Angle::radians(0.0).unwrap();
        assert!(approx_eq(a.degrees_value(), 0.0, DEFAULT_EPS));
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};
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

            prop_assume!(!approx_eq(a, b, DEFAULT_EPS));


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

    proptest! {
        #[test]
        fn degrees_and_radians_are_consistent(value in -1.0e6f64..1.0e6f64) {

            let deg = Angle::degrees(value).unwrap();
            let rad = Angle::radians(value.to_radians()).unwrap();

            prop_assert!(approx_eq(
                deg.radians_value(),
                rad.radians_value(),
                DEFAULT_EPS
            ));
        }
    }
}
