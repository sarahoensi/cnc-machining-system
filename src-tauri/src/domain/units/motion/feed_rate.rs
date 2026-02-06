// domain/units/motion/feed_rate.rs

use crate::domain::units::errors::UnitError;

/// Feed rate stored as mm/min.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct FeedRate(f64);

impl FeedRate {
    pub fn mm_per_min(value: f64) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NotFinite("FeedRate"));
        }
        if value <= 0.0 {
            return Err(UnitError::NonPositiveValue("FeedRate"));
        }
        Ok(Self(value))
    }

    pub fn mm_per_min_value(self) -> f64 {
        self.0
    }
}
