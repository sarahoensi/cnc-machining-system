// domain/units/length/length.rs

use crate::domain::units::{UnitsError, core::NumericError};

/// Represents a signed linear length measurement.
///
/// Stored internally in millimeters (mm).
/// Values must be finite, but may be negative.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Length(f64);

/// Represents a strictly positive linear length (> 0).
///
/// Stored internally in millimeters (mm).
/// Values must be finite and greater than zero.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PositiveLength(f64);



// ============================================================
// Length (signed)
// ============================================================

impl Length {

    fn validate_finite(value: f64) -> Result<f64, NumericError> {
        if value.is_finite() {
            Ok(value)
        } else {
            Err(NumericError::NotFinite(value))
        }
    }

    /// Creates a signed length in millimeters.
    pub fn mm(value: f64) -> Result<Self, UnitsError> {
        Ok(Self(Self::validate_finite(value)?))
    }

    /// Creates a signed length from inches.
    pub fn inches(value: f64) -> Result<Self, UnitsError> {
        Ok(Self(Self::validate_finite(value)? * 25.4))
    }

    /// Returns value in millimeters.
    pub fn mm_value(self) -> f64 {
        self.0
    }

    /// Returns value in inches.
    pub fn inches_value(self) -> f64 {
        self.0 / 25.4
    }

    /// Attempts to convert to PositiveLength.
    pub fn try_into_positive(self) -> Result<PositiveLength, UnitsError> {
        PositiveLength::mm(self.0)
    }
}



// ============================================================
// PositiveLength (> 0)
// ============================================================

impl PositiveLength {

    fn validate_positive(value: f64) -> Result<f64, NumericError> {
        if !value.is_finite() {
            return Err(NumericError::NotFinite(value));
        }
        if value <= 0.0 {
            return Err(NumericError::NonPositive(value));
        }
        Ok(value)
    }

    /// Creates a strictly positive length in millimeters.
    pub fn mm(value: f64) -> Result<Self, UnitsError> {
        Ok(Self(Self::validate_positive(value)?))
    }

    /// Creates from inches.
    pub fn inches(value: f64) -> Result<Self, UnitsError> {
        Ok(Self(Self::validate_positive(value)? * 25.4))
    }

    /// Returns value in millimeters.
    pub fn mm_value(self) -> f64 {
        self.0
    }

    /// Returns value in inches.
    pub fn inches_value(self) -> f64 {
        self.0 / 25.4
    }

    /// Converts to signed Length.
    pub fn as_length(self) -> Length {
        Length(self.0)
    }
}



// ============================================================
// Arithmetic for Length (signed)
// ============================================================

impl std::ops::Add for Length {
    type Output = Length;
    fn add(self, rhs: Length) -> Length {
        Length(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Length {
    type Output = Length;
    fn sub(self, rhs: Length) -> Length {
        Length(self.0 - rhs.0)
    }
}

impl std::ops::Mul<f64> for Length {
    type Output = Length;
    fn mul(self, rhs: f64) -> Length {
        Length(self.0 * rhs)
    }
}

impl std::ops::Div<f64> for Length {
    type Output = Length;
    fn div(self, rhs: f64) -> Length {
        Length(self.0 / rhs)
    }
}



// ============================================================
// Arithmetic for PositiveLength
// ============================================================

impl std::ops::Mul<f64> for PositiveLength {
    type Output = Length;
    fn mul(self, rhs: f64) -> Length {
        Length(self.0 * rhs)
    }
}

impl std::ops::Div<f64> for PositiveLength {
    type Output = Length;
    fn div(self, rhs: f64) -> Length {
        Length(self.0 / rhs)
    }
}

// ------------------- TESTS -------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};

    // --- Validation ---
/*/
    #[test]
    fn rejects_nan() {
        assert!(Length::mm(f64::NAN).is_err());
        assert!(Length::mm_non_negative(f64::NAN).is_err());
        assert!(Length::mm_positive(f64::NAN).is_err());
    }

    #[test]
    fn rejects_infinity() {
        assert!(Length::mm(f64::INFINITY).is_err());
        assert!(Length::mm(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn non_negative_rules() {
        assert!(Length::mm_non_negative(0.0).is_ok());
        assert!(Length::mm_non_negative(-1.0).is_err());
    }

    #[test]
    fn positive_rules() {
        assert!(Length::mm_positive(0.0).is_err());
        assert!(Length::mm_positive(1.0).is_ok());
    }
*/
    // --- Conversion ---

    #[test]
    fn inches_round_trip() {
        let original = 12.345;

        let l = Length::inches(original).unwrap();
        let result = l.inches_value();

        assert!(approx_eq(original, result, DEFAULT_EPS));
    }

    #[test]
    fn mm_value_round_trip() {
        let l = Length::mm(42.0).unwrap();
        assert!(approx_eq(l.mm_value(), 42.0, DEFAULT_EPS));
    }

    // --- Arithmetic ---

    #[test]
    fn add_sub_identity() {
        let a = Length::mm(10.0).unwrap();
        let b = Length::mm(5.0).unwrap();

        let result = (a + b) - b;

        assert!(approx_eq(a.mm_value(), result.mm_value(), DEFAULT_EPS));
    }

    #[test]
    fn mul_div_identity() {
        let a = Length::mm(7.5).unwrap();

        let result = (a * 3.0) / 3.0;

        assert!(approx_eq(a.mm_value(), result.mm_value(), DEFAULT_EPS));
    }

    #[test]
    fn ordering_preserved() {
        let a = Length::mm(5.0).unwrap();
        let b = Length::mm(10.0).unwrap();

        assert!(a < b);
        assert!(a + a < b + a);
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use crate::test_utils::approx::{approx_eq, DEFAULT_EPS};
    use proptest::prelude::*;

    // --- inches round trip ---
    proptest! {
        #[test]
        fn inches_round_trip_property(value in -1.0e6f64..1.0e6f64) {
            let l = Length::inches(value).unwrap();
            let result = l.inches_value();

            prop_assert!(approx_eq(value, result, DEFAULT_EPS));
        }
    }

    // --- add/sub invariant ---
    proptest! {
        #[test]
        fn add_sub_identity(a in -1.0e6f64..1.0e6f64,
                            b in -1.0e6f64..1.0e6f64) {

            let a = Length::mm(a).unwrap();
            let b = Length::mm(b).unwrap();

            let result = (a + b) - b;

            prop_assert!(approx_eq(
                a.mm_value(),
                result.mm_value(),
                DEFAULT_EPS
            ));
        }
    }

    // --- mul/div invariant ---
    proptest! {
        #[test]
        fn mul_div_identity(a in -1.0e6f64..1.0e6f64,
                            k in -1.0e3f64..1.0e3f64) {

            prop_assume!(!approx_eq(k, 0.0, DEFAULT_EPS));

            let a = Length::mm(a).unwrap();
            let result = (a * k) / k;

            prop_assert!(approx_eq(
                a.mm_value(),
                result.mm_value(),
                DEFAULT_EPS
            ));
        }
    }

    // --- arithmetic finite ---
    proptest! {
        #[test]
        fn arithmetic_stays_finite(a in -1.0e6f64..1.0e6f64,
                                   b in -1.0e6f64..1.0e6f64) {

            let a = Length::mm(a).unwrap();
            let b = Length::mm(b).unwrap();

            let r = a + b;

            prop_assert!(r.mm_value().is_finite());
        }
    }
}
