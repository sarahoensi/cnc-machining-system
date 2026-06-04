// domain/units/motion/feed_rate.rs

use crate::domain::units::{PositiveScalar, UnitsError};

/// Represents linear feed rate.
///
/// Stored internally as millimeters per minute (mm/min).
/// Values must be finite and strictly positive.
#[must_use]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct FeedRate(PositiveScalar);

impl FeedRate {
    /// Internal constructor used by domain math.
    #[allow(dead_code)]
    pub(crate) fn mm_per_min_unchecked(value: f64) -> Self {
        Self(PositiveScalar::new_unchecked(value))
    }

    pub fn mm_per_min(value: f64) -> Result<Self, UnitsError> {
        Ok(Self(PositiveScalar::new(value)?))
    }

    pub fn mm_per_min_value(self) -> f64 {
        self.0.value()
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
        assert!(FeedRate::mm_per_min(f64::NAN).is_err());
    }

    #[test]
    fn rejects_infinity() {
        assert!(FeedRate::mm_per_min(f64::INFINITY).is_err());
        assert!(FeedRate::mm_per_min(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn rejects_zero() {
        assert!(FeedRate::mm_per_min(0.0).is_err());
    }

    #[test]
    fn rejects_negative() {
        assert!(FeedRate::mm_per_min(-1.0).is_err());
    }

    // --- Value round trip ---

    #[test]
    fn value_round_trip() {
        let f = FeedRate::mm_per_min(150.0).unwrap();

        assert!(approx_eq(f.mm_per_min_value(), 150.0, DEFAULT_EPS));
    }

    // --- Ordering ---

    #[test]
    fn ordering_works() {
        let a = FeedRate::mm_per_min(100.0).unwrap();
        let b = FeedRate::mm_per_min(200.0).unwrap();

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
        fn round_trip_property(value in 1e-9f64..1e6f64) {
            let f = FeedRate::mm_per_min(value).unwrap();

            prop_assert!(approx_eq(
                f.mm_per_min_value(),
                value,
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

            let a = FeedRate::mm_per_min(a).unwrap();
            let b = FeedRate::mm_per_min(b).unwrap();

            prop_assert!(a < b);
        }
    }

    // --- finite invariant ---
    proptest! {
        #[test]
        fn value_stays_finite(value in 1e-9f64..1e6f64) {
            let f = FeedRate::mm_per_min(value).unwrap();

            prop_assert!(f.mm_per_min_value().is_finite());
        }
    }
}
