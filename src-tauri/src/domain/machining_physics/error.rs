// domain/machining_physics/error.rs

use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum MachiningPhysicsError {

    /// Tool diameter was zero or negative.
    #[error("Tool diameter must be positive (got {value_mm} mm)")]
    InvalidDiameter {
        value_mm: f64,
    },

    /// Tooth count was zero.
    #[error("Tooth count must be greater than zero (got {value})")]
    InvalidToothCount {
        value: u32,
    },

    /// RPM computed or provided was not positive or finite.
    #[error("RPM must be positive and finite (got {value})")]
    InvalidRpm {
        value: f64,
    },

    /// Feed rate computed was not positive or finite.
    #[error("Feed rate must be positive and finite (got {value})")]
    InvalidFeedRate {
        value: f64,
    },

    /// Chip load computed was not positive or finite.
    #[error("Chip load must be positive and finite (got {value})")]
    InvalidChipLoad {
        value: f64,
    },

    /// Division by zero or near-zero occurred.
    #[error("Division by zero")]
    DivisionByZero,

    /// Floating point instability (e.g. overflow / NaN).
    #[error("Numerical instability detected")]
    NumericalInstability,
}