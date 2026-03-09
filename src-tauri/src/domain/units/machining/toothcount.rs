// domain/units/machining/toothcount.rs

use crate::domain::units::{
    UnitsError, core::NumericError,
};

#[must_use]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ToothCount(i32);

impl ToothCount {

    /// Internal constructor used by domain math.
    #[allow(dead_code)]
    pub(crate) fn new_unchecked(value: i32) -> Self {
        debug_assert!(value > 0);
        Self(value)
    }

    pub fn new(value: i32) -> Result<Self, UnitsError> {
        if value == 0 {
            return Err(NumericError::NonPositive(value as f64).into());
        }

        Ok(Self(value))
    }

    pub fn value(self) -> i32 {
        self.0
    }
}