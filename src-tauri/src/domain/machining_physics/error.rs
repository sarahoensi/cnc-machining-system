// domain/machining_physics/error.rs

// domain/machining_physics/error.rs

use thiserror::Error;
use crate::domain::units::UnitsError;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum MachiningPhysicsError {

    /// Bubble up unit-level invariant violations.
    #[error(transparent)]
    Unit(#[from] UnitsError),

    /// Division by zero or near-zero occurred.
    #[error("Division by zero")]
    DivisionByZero,

    /// Floating point instability (e.g. overflow / NaN).
    #[error("Numerical instability detected")]
    NumericalInstability,
}