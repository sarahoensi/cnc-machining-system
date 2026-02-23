// domain/geometry/right_triangle/error.rs

// domain/geometry/right_triangle/error.rs

#[derive(Debug, Clone, PartialEq)]
pub enum RightTriangleError {
    HypotenuseTooShort {
        leg: f64,
        hypotenuse: f64,
    },

    LegNotPositive {
        value: f64,
    },

    HypotenuseNotPositive {
        value: f64,
    },

    AngleNotAcute {
        degrees: f64,
    },

    DivisionByZero,

    NumericalInstability,
}

use std::fmt;

impl fmt::Display for RightTriangleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for RightTriangleError {}