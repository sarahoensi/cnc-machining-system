// domain/units/core/positive_scalar.rs

use crate::domain::units::core::NumericError;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PositiveScalar(f64);

impl PositiveScalar {

    /// Internal constructor used by domain math.
    pub(crate) fn new_unchecked(value: f64) -> Self {
        debug_assert!(value.is_finite());
        debug_assert!(value > 0.0);
        Self(value)
    }

    pub fn new(value: f64) -> Result<Self, NumericError> {
        if !value.is_finite() {
            return Err(NumericError::NotFinite(value));
        }
        if value <= 0.0 {
            return Err(NumericError::NonPositive(value));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> f64 {
        self.0
    }
}