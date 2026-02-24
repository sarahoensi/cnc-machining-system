// domain/geometry/right_triangle/error.rs

use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum RightTriangleError {
    #[error("Hypotenuse ({hypotenuse}) must be greater than leg ({leg})")]
    HypotenuseTooShort {
        leg: f64,
        hypotenuse: f64,
    },

    #[error("Leg must be positive (got {value})")]
    LegNotPositive {
        value: f64,
    },

    #[error("Hypotenuse must be positive (got {value})")]
    HypotenuseNotPositive {
        value: f64,
    },

    #[error("Angle must be between 0 and 90 degrees (got {degrees})")]
    AngleNotAcute {
        degrees: f64,
    },

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Numerical instability detected")]
    NumericalInstability,
}