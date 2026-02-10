// domain/machining_physics/tools.rs

use crate::domain::UnitError;
use crate::domain::Diameter;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ToothCount(u32);

impl ToothCount {
    pub fn new(value: u32) -> Result<Self, UnitError> {
        if value == 0 {
            return Err(UnitError::NonPositiveValue("ToothCount"));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tool {
    diameter: Diameter,
    teeth: ToothCount,
}

impl Tool {
    pub fn new(diameter: Diameter, teeth: ToothCount) -> Self {
        Self { diameter, teeth }
    }

    pub fn diameter(&self) -> Diameter {
        self.diameter
    }

    pub fn teeth(&self) -> ToothCount {
        self.teeth
    }
}
