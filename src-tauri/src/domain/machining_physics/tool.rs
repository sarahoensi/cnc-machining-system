// domain/machining_physics/tools.rs

use crate::domain::{
    units::{Diameter, UnitError},
};


/// Number of cutting edges (teeth) on a tool.
///
/// Must always be greater than zero.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ToothCount(u32);

/// Creates a new tooth count.
    ///
    /// Returns an error if the value is zero.
impl ToothCount {
    pub fn new(value: u32) -> Result<Self, UnitError> {
        if value == 0 {
            return Err(UnitError::NonPositiveValue("ToothCount"));
        }
        Ok(Self(value))
    }

    /// Returns the number of teeth.
    pub fn value(self) -> u32 {
        self.0
    }
}

/// Represents a cutting tool used in machining calculations.
///
/// A tool is defined by:
///
/// - Tool diameter
/// - Number of cutting teeth
///
/// This information is used when calculating chip load,
/// feed rate, and other machining parameters.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tool {
    diameter: Diameter,
    teeth: ToothCount,
}

impl Tool {
    /// Creates a new machining tool definition.
    pub fn new(diameter: Diameter, teeth: ToothCount) -> Self {
        Self { diameter, teeth }
    }

    
    /// Returns the tool's diameter.
    pub fn diameter(&self) -> Diameter {
        self.diameter
    }

    /// Returns the number of cutting teeth on the tool.
    pub fn teeth(&self) -> ToothCount {
        self.teeth
    }
}
