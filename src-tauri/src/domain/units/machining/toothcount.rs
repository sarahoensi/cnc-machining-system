// domain/units/machining/toothcount.rs

use crate::domain::units::{
    core::NumericError,
    machining::MachiningUnitError,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ToothCount(u32);

impl ToothCount {

    pub fn new(value: u32) -> Result<Self, MachiningUnitError> {
        if value == 0 {
            return Err(NumericError::NonPositive(value as f64).into());
        }

        Ok(Self(value))
    }

    pub fn value(self) -> u32 {
        self.0
    }
}