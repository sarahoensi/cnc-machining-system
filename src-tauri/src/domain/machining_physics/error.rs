// domain/machining_physics/error.rs

#[derive(Debug, Clone, PartialEq)]
pub enum MachiningPhysicsError {

    /// Tool diameter was zero or negative (should not happen if Diameter is valid).
    InvalidDiameter {
        value_mm: f64,
    },

    /// Tooth count was zero.
    InvalidToothCount {
        value: u32,
    },

    /// RPM computed or provided was not positive or finite.
    InvalidRpm {
        value: f64,
    },

    /// Feed rate computed was not positive or finite.
    InvalidFeedRate {
        value: f64,
    },

    /// Chip load computed was not positive or finite.
    InvalidChipLoad {
        value: f64,
    },

    /// Division by zero or near-zero occurred.
    DivisionByZero,

    /// Floating point instability (e.g. overflow / NaN).
    NumericalInstability,
}

use std::fmt;

impl fmt::Display for MachiningPhysicsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for MachiningPhysicsError {}