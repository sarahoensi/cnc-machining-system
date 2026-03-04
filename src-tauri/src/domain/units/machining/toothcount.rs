// domain/units/machining/toothcount.rs

use crate::domain::units::{
    UnitsError, core::NumericError,
};

#[must_use]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ToothCount(u32);

impl ToothCount {

    /// Internal constructor used by domain math.
    #[allow(dead_code)]
    pub(crate) fn new_unchecked(value: u32) -> Self {
        debug_assert!(value > 0);
        Self(value)
    }

    pub fn new(value: u32) -> Result<Self, UnitsError> {
        if value == 0 {
            return Err(NumericError::NonPositive(value as f64).into());
        }

        Ok(Self(value))
    }

    pub fn value(self) -> u32 {
        self.0
    }
}